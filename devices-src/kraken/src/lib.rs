use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use neon::prelude::*;
use once_cell::sync::Lazy;
use rusb::{open_device_with_vid_pid, DeviceHandle, GlobalContext};
mod statics;
use crate::statics::statics::*;
use std::sync::Mutex;

// static FRAME_CACHE: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| {
//     let frames = Vec::new();
//     Mutex::new(frames)
// });

static SWITCHY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| {
    let mut switchy = Vec::new();
    switchy.push(16 as u8);
    Mutex::new(switchy)
});

fn dissect_error(error: rusb::Error) {
    let reason = error.to_string();
    println!("{:?}", reason);
}

static BULK_HANDLE: Lazy<DeviceHandle<GlobalContext>> = Lazy::new(|| {
    let mut device = open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID).unwrap();
    let whathappened = device.claim_interface(BULK_INTERFACE);
    let _whathappened2 = device.claim_interface(HID_INTERFACE);
    match whathappened {
        Ok(_ok) => print!(""),
        Err(err) => dissect_error(err),
    }
    device
});

static SHOULD_SEND: Lazy<Mutex<Vec<bool>>> = Lazy::new(|| {
    let mut should_send = Vec::new();
    should_send.push(false);
    Mutex::new(should_send)
});

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("send_frame", image_passer)?;
    cx.export_function("open_device", open_device)?;
    cx.export_function("close_device", close_device)?;
    Ok(())
}

fn open_device(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut send_bool = SHOULD_SEND.lock().expect("Failed to get lock!");
    send_bool.remove(0);
    send_bool.push(true);
    // std::thread::spawn(send_image_thread);
    Ok(cx.string("s"))
}

fn close_device(mut cx: FunctionContext) -> JsResult<JsString> {
    let mut send_bool = SHOULD_SEND.lock().expect("Failed to get lock!");
    send_bool.remove(0);
    send_bool.push(false);
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(cx.string("s"))
}

// SEND IMAGE
fn image_passer(mut cx: FunctionContext) -> JsResult<JsString> {
    let base_64 = cx.argument::<JsString>(0)?;
    let image = general_purpose::STANDARD
        .decode(base_64.value(&mut cx))
        .unwrap();
    let convert = convert_image(image);
    // let mut img_vec = FRAME_CACHE.lock().expect("Failed to get lock!");
    // img_vec.push(product);
    // drop(img_vec);
    let lastone = send_frame(
        &BULK_HANDLE,
        &convert,
        SWITCHY.lock().unwrap().get(0).unwrap(),
    );
    SWITCHY.lock().unwrap().remove(0);
    SWITCHY.lock().unwrap().push(lastone);
    Ok(cx.string("done"))
}

fn convert_image(image: Vec<u8>) -> Vec<u8> {
    let convert = DynamicImage::ImageRgb8(image::load_from_memory(&image).unwrap().into_rgb8())
        .resize_exact(320, 320, image::imageops::FilterType::Triangle)
        .as_bytes()
        .to_vec();
    let mut product: Vec<u8> = Vec::new();
    for chunk in convert.chunks(3) {
        product.extend(chunk);
        product.push(0x00 as u8);
    }
    product
}

fn zeropad_vector(to_pad: &[u8], intended_length: usize) -> Vec<u8> {
    let mut vector = to_pad.to_vec();
    vector.resize(intended_length, 0x00);
    vector
}

fn bulk_write(handle: &DeviceHandle<GlobalContext>, buffer: &[u8]) {
    handle
        .write_bulk(
            BULK_ENDPOINT,
            &zeropad_vector(buffer, BULK_WRITE_LENGTH),
            TEN_MS,
        )
        .expect("Failed to write to bulk endpoint!");
}

fn query_buckets(handle: &DeviceHandle<GlobalContext>) -> Vec<Vec<u8>> {
    let mut buckets: Vec<Vec<u8>> = Vec::new();
    for i in 0..16 {
        let response = write_read(handle, &[0x30, 0x04, i as u8]);
        buckets.push(response.to_vec());
    }
    buckets
}

fn find_free_bucket(buckets: &Vec<Vec<u8>>, lastone: &u8) -> u8 {
    for i in 0..16 {
        if buckets[i][15..]
            .iter()
            .all(|item| item.to_owned() == 0x00 as u8)
        {
            if i != lastone.to_owned() as usize {
                return i as u8;
            }
        }
    }
    0xFF
}

fn prepare_bucket(index: u8, filled: bool, handle: &DeviceHandle<GlobalContext>) -> u8 {
    let delete_response = delete_bucket(index, handle);
    if !delete_response {
        return prepare_bucket(index + 1, true, handle);
    } else {
        if filled {
            return prepare_bucket(index, false, handle);
        }
    }
    index
}

fn get_bucket_memory_offset(buckets: &Vec<Vec<u8>>, index: u8, packet_count: usize) -> i16 {
    let current_bucket = buckets.get(index as usize).unwrap();
    let bucket_offset = i16::from_le_bytes([current_bucket[17], current_bucket[18]]);
    let bucket_size = i16::from_le_bytes([current_bucket[19], current_bucket[20]]);

    if packet_count <= bucket_size as usize {
        return bucket_offset;
    }

    let mut min_occupied_byte = bucket_offset;
    let mut max_occupied_byte: i16 = 0;
    let mut existing_bucket_within_range = false;

    for i in 0..16 {
        let bucket = buckets.get(i as usize).unwrap();
        let start_byte = i16::from_le_bytes([bucket[17], bucket[18]]);
        let end_byte = start_byte - i16::from_le_bytes([bucket[19], bucket[20]]);
        if end_byte > max_occupied_byte {
            max_occupied_byte = end_byte;
        }
        if start_byte < min_occupied_byte {
            min_occupied_byte = start_byte;
        }
        if (start_byte > bucket_offset && start_byte < bucket_offset + packet_count as i16)
            || (start_byte < bucket_offset && end_byte > start_byte)
            || (start_byte == bucket_offset && i as u8 != index)
        {
            existing_bucket_within_range = true;
        }
    }

    if !existing_bucket_within_range {
        return bucket_offset;
    }

    if max_occupied_byte as usize + packet_count < LCD_TOTAL_MEMORY {
        return max_occupied_byte;
    }

    if packet_count < min_occupied_byte as usize {
        return 0x000;
    }

    0xFFF
}

fn switch_bucket(index: u8, mut mode: u8, handle: &DeviceHandle<GlobalContext>) -> bool {
    if mode == 0xFF {
        mode = 0x4;
    }
    let reponse = write_read(handle, &[0x38, 0x1, mode, index]);
    reponse[14] == 0x1
}

fn delete_all_buckets(handle: &DeviceHandle<GlobalContext>) {
    switch_bucket(0, 0xFF, handle);
    for i in 0..16 {
        delete_bucket(i, handle);
    }
}

fn delete_bucket(index: u8, handle: &DeviceHandle<GlobalContext>) -> bool {
    let response = write_read(handle, &[0x32, 0x02, index]);
    response[14] == 0x01
}

fn setup_bucket(
    start_index: u8,
    starting_memory_address: i16,
    memory_size: u16,
    handle: &DeviceHandle<GlobalContext>,
) -> bool {
    let response = write_read(
        handle,
        &[
            0x32,
            0x1,
            start_index,
            start_index + 1,
            (starting_memory_address & 0xFF) as u8,
            ((starting_memory_address >> 8) & 0xFF) as u8,
            (memory_size & 0xff) as u8,
            ((memory_size >> 8) & 0xFF) as u8,
            0x1,
        ],
    );
    response[14] == 0x01
}

fn write_read(handle: &DeviceHandle<GlobalContext>, buffer: &[u8]) -> [u8; READ_LENGTH] {
    handle
        .write_interrupt(
            INTERRUPT_ENDPOINT_OUT,
            &zeropad_vector(buffer, WRITE_LENGTH),
            TEN_MS,
        )
        .expect("Failed to write to device!");
    let mut read_buf = [0u8; READ_LENGTH];
    handle
        .read_interrupt(INTERRUPT_ENDPOINT_IN, &mut read_buf, TEN_MS)
        .expect("Failed to read response!");
    read_buf
}

fn send_frame(handle: &DeviceHandle<GlobalContext>, frame: &[u8], lastone: &u8) -> u8 {
    write_read(&handle, &[0x36, 0x03]);

    let buckets = query_buckets(&handle);

    handle
        .write_interrupt(
            INTERRUPT_ENDPOINT_OUT,
            &zeropad_vector(&[0x20, 0x03], WRITE_LENGTH),
            TEN_MS,
        )
        .expect("Failed to write to device!");

    write_read(&handle, &[0x74, 0x01]);
    write_read(&handle, &[0x70, 0x01]);
    write_read(&handle, &[0x74, 0x01]);

    let mut index = find_free_bucket(&buckets, &lastone);

    index = prepare_bucket(
        if index != 0xFF { index } else { 0x00 },
        index == 0xFF,
        &handle,
    );

    let mut bucket_memory_start = get_bucket_memory_offset(&buckets, index, 400);

    if bucket_memory_start == 0xFFF {
        delete_all_buckets(&handle);
        index = 0;
        bucket_memory_start = 0x0000;
    }

    if !setup_bucket(index, bucket_memory_start, 400 as u16, &handle) {
        println!("Failed to setup bucket for data transfer!");
    }

    write_read(&handle, &[0x36, 0x01, index]);

    bulk_write(
        &handle,
        &[
            0x12, 0xFA, 0x01, 0xE8, 0xAB, 0xCD, 0xEF, 0x98, 0x76, 0x54, 0x32, 0x10, 0x02, 0x0, 0x0,
            0x0, 0x0, 0x40, 0x96,
        ],
    );

    for chunk in frame.chunks(BULK_WRITE_LENGTH) {
        bulk_write(&handle, chunk);
    }

    handle
        .write_interrupt(
            INTERRUPT_ENDPOINT_OUT,
            &zeropad_vector(&[0x36, 0x02], WRITE_LENGTH),
            TEN_MS,
        )
        .expect("Failed to complete transfer!");

    let _resp = switch_bucket(index, 0x4, &handle);

    index
}
