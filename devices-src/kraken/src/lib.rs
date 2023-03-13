extern crate hidapi;
use base64::{engine::general_purpose, Engine as _};
use hidapi::HidDevice;
use image::DynamicImage;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::{convert::TryFrom, sync::Mutex};

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1e71, 0x3008)
.unwrap();}

const READ_LENGTH: usize = 64;
static WRITE_LENGTH: usize = 64;
static MAX_READ_ATTEMPTS: usize = 12;
static BULK_WRITE_LENGTH: usize = 512;
static LCD_TOTAL_MEMORY: usize = 24320;

static FRAME_CACHE: Lazy<Mutex<Vec<Vec<u8>>>> = Lazy::new(|| {
    let frames = Vec::new();
    Mutex::new(frames)
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
        if img_vec.len() == 0 {
            drop(img_vec);
            std::thread::sleep(std::time::Duration::from_millis(10));
        } else {
            let image = img_vec.get(0).to_owned();
            img_vec.remove(0);
            drop(img_vec);
            GLOBAL_DATA.with(|hid| {
                // READ BUCKET INFO
                let mut buckets: Vec<Vec<u8>> = Vec::new();
                for i in 0..15 as u8 {
                    hid.write(&[0x30, 0x04, i]).unwrap();
                    let mut buf = [0u8; READ_LENGTH];
                    let _result = hid.read(&mut buf);
                    buckets.push(Vec::try_from(buf).expect("Failed to convert into u8!"));
                }
            });
        }
    }
    return;
}
