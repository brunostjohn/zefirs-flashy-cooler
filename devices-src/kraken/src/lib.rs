extern crate hidapi;
use base64::{engine::general_purpose, Engine as _};
use hidapi::HidDevice;
use image::DynamicImage;
use neon::prelude::*;
use once_cell::sync::Lazy;
use rusb::{open_device_with_vid_pid, DeviceHandle, GlobalContext};
use std::{convert::TryFrom, sync::Mutex, time::Duration};

static VENDOR_ID: u16 = 0x1e71;
static PRODUCT_ID: u16 = 0x3008;

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1e71, 0x3008)
.unwrap();}

const READ_LENGTH: usize = 64;
const WRITE_LENGTH: usize = 64;
static MAX_READ_ATTEMPTS: usize = 12;
static BULK_WRITE_LENGTH: usize = 512;
static LCD_TOTAL_MEMORY: usize = 24320;

static FRAME_CACHE: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| {
    let frames = Vec::new();
    Mutex::new(frames)
});

static BULK_HANDLE: Lazy<DeviceHandle<GlobalContext>> = Lazy::new(|| {
    let mut device = open_device_with_vid_pid(VENDOR_ID, PRODUCT_ID).unwrap();
    // device.claim_interface(0x01);
    device.claim_interface(0x00);
    return device;
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
    GLOBAL_DATA.with(|hid| {
        // do io ops
    });
    std::thread::sleep(std::time::Duration::from_millis(5));
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
    GLOBAL_DATA.with(|hid| {
        // do io ops
    });
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
    while (SHOULD_SEND.lock().expect("Failed to get lock!").get(0))
        .unwrap()
        .to_owned()
        == true
    {
        let mut img_vec = FRAME_CACHE.lock().expect("Failed to get lock!");
        let vec_copy = img_vec.clone();
        let mut image = vec_copy.get(0).clone();
        if vec_copy.len() == 0 {
            std::thread::sleep(std::time::Duration::from_millis(10));
        } else {
            img_vec.remove(0);
            drop(img_vec);
            let img = image.clone().unwrap();
            GLOBAL_DATA.with(|hid| {
                // SAY HELLO
                let _hello = hid.write(&[0x36, 0x03]).expect("Failed to say hello.");
                // READ BUCKET INFO
                let mut buckets: Vec<Vec<u8>> = Vec::new();
                for i in 0..15 as u8 {
                    hid.write(&[0x30, 0x04, i]).unwrap();
                    let mut buf = [0u8; READ_LENGTH];
                    let _result = hid.read(&mut buf);
                    buckets.push(Vec::try_from(buf).expect("Failed to convert into u8!"));
                }
                let _init1 = hid
                    .write(&[0x20, 0x03])
                    .expect("Failed to send unknown packet 1.");
                let _keepalive1 = hid
                    .write(&[0x74, 0x01])
                    .expect("Failed to send keepalive 1!");
                let _init2 = hid
                    .write(&[0x70, 0x01])
                    .expect("Failed to send unknown packet 2!");
                let _keepalive2 = hid
                    .write(&[0x74, 0x01])
                    .expect("Failed to send keepalive 2!");
                // FIND FREE BUCKET
                let mut free_bucket = 16;
                for i in 0..15 as u8 {
                    if buckets.get(i as usize).unwrap()[15..]
                        .to_vec()
                        .iter()
                        .all(|byte| byte.clone() == 0x00 as u8)
                    {
                        free_bucket = i;
                        println!("Found bucket {:?}", i);
                        break;
                    }
                }
                if free_bucket != 16 {
                    // no free bucket = dropped frame
                    // PREPARE BUCKET
                    let _response = hid.write(&[0x32, 0x02, free_bucket]).unwrap();
                    let mut buf = [0u8; READ_LENGTH];
                    let _result = hid.read(&mut buf).unwrap();
                    // check for 0x01 on i = 14
                    if buf.get(14).unwrap().clone() == 0x01 as u8 {
                        // TODO, fix this shitty temp solution, for now it drops frames every time something goes wrong. this is a fucking joke and cannot stay like this.
                        // if we're lucky, now we should have a bucket that can actually b used
                        let total_size = img.len();
                        let packet_count = img.chunks(BULK_WRITE_LENGTH).len();
                        let cur_bucket = buckets.get(free_bucket as usize).unwrap();
                        let bucket_offset = ((cur_bucket[18] as u16) << 8) | cur_bucket[17] as u16;
                        let bucket_size = ((cur_bucket[18] as u16) << 8) | cur_bucket[17] as u16;
                        let mut can_we_proceed = false;
                        // HERE IM MAKING A BAD ASSUMPTION, THIS IS JUST A POC
                        if packet_count <= bucket_size as usize {
                            can_we_proceed = true;
                        } else {
                            print!("Rejected because: ");
                            print!("{:?} ", packet_count);
                            print!("{:?}", bucket_size);
                        }
                        if can_we_proceed {
                            let _reponse2 = hid
                                .write(&[
                                    0x32,
                                    0x01,
                                    free_bucket,
                                    free_bucket + 1,
                                    shift_verbose_split_u16(bucket_offset)[1],
                                    shift_verbose_split_u16(bucket_offset)[0],
                                    shift_verbose_split_u16(total_size as u16)[1],
                                    shift_verbose_split_u16(total_size as u16)[0],
                                    0x01,
                                ])
                                .unwrap();
                            let _response3 = hid.write(&[0x36, 0x01, free_bucket]).unwrap();
                            // let _reponse4 = hid.write(&[
                            //     0x12, 0xFA, 0x01, 0xE8, 0xAB, 0xCD, 0xEF, 0x98, 0x76, 0x54, 0x32,
                            //     0x10, 0x02, 0x0, 0x0, 0x0, 0x0, 0x40, 0x06,
                            // ]);
                            let _handle2res = BULK_HANDLE
                                .write_bulk(
                                    0x00,
                                    &[
                                        0x12, 0xFA, 0x01, 0xE8, 0xAB, 0xCD, 0xEF, 0x98, 0x76, 0x54,
                                        0x32, 0x10, 0x02, 0x0, 0x0, 0x0, 0x0, 0x40, 0x96,
                                    ],
                                    Duration::from_millis(10),
                                )
                                .unwrap();
                            for chunk in img.chunks(BULK_WRITE_LENGTH) {
                                println!("I AM ACTUALLY WRITING THE IMAGE");
                                let _result5 = BULK_HANDLE
                                    .write_bulk(0x00, chunk, Duration::from_millis(10))
                                    .unwrap();
                            }
                            let _finalres = hid.write(&[0x36, 0x02]).unwrap();
                            let _switch = hid.write(&[0x38, 0x01, 0x04, free_bucket]).unwrap();
                        }
                    }
                }
            });
        }
    }
    return;
}

fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
    let high_byte: u8 = (short_16 >> 8) as u8;
    let low_byte: u8 = (short_16 & 0xff) as u8;

    return [high_byte, low_byte];
}
