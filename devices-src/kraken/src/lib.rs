extern crate hidapi;
use base64::{engine::general_purpose, Engine as _};
use hidapi::HidDevice;
use image::DynamicImage;
use neon::prelude::*;

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1e71, 0x3008)
.unwrap();}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("send_frame", image_passer)?;
    cx.export_function("open_device", open_device)?;
    cx.export_function("close_device", close_device)?;
    Ok(())
}

fn open_device(mut cx: FunctionContext) -> JsResult<JsString> {
    GLOBAL_DATA.with(|hid| {
        // hid.send_feature_report(&[0x03, 0x19, 0x40, 0x01, 0x3b, 0x00, 0x77, 0x03])
        //     .expect("Failed to open LCD!");
    });
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(cx.string("s"))
}

fn close_device(mut cx: FunctionContext) -> JsResult<JsString> {
    std::thread::sleep(std::time::Duration::from_millis(5));
    // GLOBAL_DATA.with(|hid| {
    //     hid.send_feature_report(&[0x03, 0x1e, 0x40, 0x01, 0x43, 0x00, 0x69, 0x00])
    //         .expect("Failed to close LCD!");
    //     drop(hid);
    // });
    Ok(cx.string("s"))
}

// SEND IMAGE
fn image_passer(mut cx: FunctionContext) -> JsResult<JsString> {
    let image = std::fs::read("./image.jpeg").unwrap();
    let convert = DynamicImage::ImageRgb8(image::load_from_memory(&image).unwrap().into_rgb8())
        .resize_exact(320, 320, image::imageops::FilterType::Triangle)
        .as_bytes()
        .to_vec();
    let mut product: Vec<u8> = Vec::new();
    for chunk in convert.chunks(3) {
        product.push(0xFF as u8);
        product.extend(chunk);
    }
    GLOBAL_DATA.with(|hid| {
        send_image(hid, product);
    });
    Ok(cx.string("done"))
}

pub fn send_image(handle: &hidapi::HidDevice, image: Vec<u8>) {
    let device = handle;
}
