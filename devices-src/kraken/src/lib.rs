use base64::{engine::general_purpose, Engine as _};
use image::DynamicImage;
use neon::prelude::*;
use once_cell::sync::Lazy;
use rusb::{open_device_with_vid_pid, DeviceHandle, GlobalContext};
mod statics;
use crate::statics::statics::*;
use std::{convert::TryFrom, sync::Mutex};

static FRAME_CACHE: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| {
    let frames = Vec::new();
    Mutex::new(frames)
});

static SWITCHY: Lazy<Mutex<Vec<u8>>> = Lazy::new(|| {
    let mut switchy = Vec::new();
    switchy.push(0x00 as u8);
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
        Ok(_ok) => println!("weregood"),
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
    std::thread::spawn(send_image_thread);
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
    let convert = DynamicImage::ImageRgb8(image::load_from_memory(&image).unwrap().into_rgb8())
        .resize_exact(320, 320, image::imageops::FilterType::Triangle)
        .as_bytes()
        .to_vec();
    let mut product: Vec<u8> = Vec::new();
    for chunk in convert.chunks(3) {
        product.push(0xFF as u8);
        product.extend(chunk);
    }
    let mut img_vec = FRAME_CACHE.lock().expect("Failed to get lock!");
    img_vec.push(product);
    drop(img_vec);
    Ok(cx.string("done"))
}

pub fn send_image_thread() {
    while (SHOULD_SEND
        .lock()
        .expect("Failed to get lock for should send!")
        .get(0))
    .unwrap()
    .to_owned()
        == true
    {
        let mut img_vec = FRAME_CACHE
            .lock()
            .expect("Failed to get lock for image vector!");
        let vec_copy = img_vec.clone();
        let image = vec_copy.get(0).clone();
        if vec_copy.len() == 0 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        } else {
            img_vec.remove(0);
            drop(img_vec);
            let img = image.clone().unwrap();
            let apply_after_set = true;
            let index: u8;
            let mut switchy = SWITCHY.lock().expect("Failed to acquire switchy");
            if switchy.get(0).unwrap() == &(0 as u8) {
                index = 1;
                switchy.remove(0);
                switchy.push(1);
            } else {
                index = 0;
            }
            query_buckets(index);
            send_delete_bucket(index);
            send_setup_bucket(index, index + 1, calculate_memory_start(index), 400);
            send_write_start_bucket(index);
            send_bulk_data_info(0x02, &BULK_HANDLE);
            for chunk in img.chunks(BULK_WRITE_LENGTH) {
                BULK_HANDLE
                    .write_bulk(BULK_ENDPOINT, chunk, TEN_MS)
                    .expect("Failed to write bulk data!");
            }
            send_write_finish_bucket(index);
            if apply_after_set {
                send_switch_bucket(index, 2);
            }
        }
    }
}

fn zeropad_vector(mut to_pad: Vec<u8>, intended_length: usize) -> Vec<u8> {
    to_pad.resize(intended_length, 0x00);
    to_pad
}

fn send_bulk_data_info(
    mode: u8,
    bulk_handle: &once_cell::sync::Lazy<DeviceHandle<GlobalContext>>,
) -> usize {
    let mut initial_data: Vec<u8> = Vec::try_from([
        0x12, 0xfa, 0x01, 0xe8, 0xab, 0xcd, 0xef, 0x98, 0x76, 0x54, 0x32, 0x10,
    ])
    .unwrap();
    initial_data.push(mode);
    initial_data = zeropad_vector(initial_data, 17);
    initial_data.extend([0x40, 0x96]);
    initial_data = zeropad_vector(initial_data, BULK_WRITE_LENGTH);
    let result = bulk_handle.write_bulk(BULK_ENDPOINT, &initial_data, TEN_MS);
    match result {
        Ok(wrote) => return wrote,
        Err(_err) => return 0 as usize,
    }
}

fn query_buckets(index: u8) -> usize {
    if index < 15 {
        let mut query_bucket: Vec<u8> = Vec::new();
        query_bucket.extend([0x30, 0x04, 0x00, index]);
        query_bucket = zeropad_vector(query_bucket, WRITE_LENGTH);
        let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &query_bucket, TEN_MS);
        match result {
            Ok(ok) => return ok,
            Err(_err) => return 0,
        }
    }
    0
}

fn send_delete_bucket(index: u8) -> usize {
    if index < 15 {
        let mut del_bucket: Vec<u8> = Vec::new();
        del_bucket.extend([0x32, 0x02, index]);
        del_bucket = zeropad_vector(del_bucket, WRITE_LENGTH);
        let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &del_bucket, TEN_MS);
        match result {
            Ok(ok) => return ok,
            Err(_err) => return 0,
        }
    }
    0
}
fn send_setup_bucket(index: u8, id: u8, memory_slot: u16, memory_slot_count: u16) -> usize {
    let mut setup_bucket: Vec<u8> = Vec::new();
    setup_bucket.extend([
        0x32,
        0x01,
        index,
        id,
        (memory_slot >> 8) as u8,
        memory_slot as u8,
        memory_slot_count as u8,
        (memory_slot_count >> 8) as u8,
        0x01,
    ]);
    setup_bucket = zeropad_vector(setup_bucket, WRITE_LENGTH);
    let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &setup_bucket, TEN_MS);
    match result {
        Ok(ok) => return ok,
        Err(_err) => return 0,
    }
}

fn calculate_memory_start(index: u8) -> u16 {
    800 * index as u16
}

fn send_write_start_bucket(index: u8) -> usize {
    if index < 15 {
        let mut start_bucket: Vec<u8> = Vec::new();
        start_bucket.extend([0x36, 0x01, index]);
        start_bucket = zeropad_vector(start_bucket, WRITE_LENGTH);
        let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &start_bucket, TEN_MS);
        match result {
            Ok(ok) => return ok,
            Err(_err) => return 0,
        }
    }
    0
}

fn send_write_finish_bucket(index: u8) -> usize {
    if index < 15 {
        let mut finish_bucket: Vec<u8> = Vec::new();
        finish_bucket.extend([0x36, 0x02, index]);
        finish_bucket = zeropad_vector(finish_bucket, WRITE_LENGTH);
        let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &finish_bucket, TEN_MS);
        match result {
            Ok(ok) => return ok,
            Err(_err) => return 0,
        }
    }
    0
}

fn send_switch_bucket(index: u8, mode: u8) -> usize {
    if index < 15 {
        let mut switch_bucket: Vec<u8> = Vec::new();
        switch_bucket.extend([0x38, 0x01, mode, index]);
        switch_bucket = zeropad_vector(switch_bucket, WRITE_LENGTH);
        let result = BULK_HANDLE.write_interrupt(INTERRUPT_ENDPOINT, &switch_bucket, TEN_MS);
        match result {
            Ok(ok) => return ok,
            Err(_err) => return 0,
        }
    }
    0
}
