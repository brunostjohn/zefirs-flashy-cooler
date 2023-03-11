extern crate hidapi;
use base64::{engine::general_purpose, Engine as _};
use core::time;
use hidapi::HidDevice;
use neon::prelude::*;
use std::convert::TryFrom;

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1b1c, 0x0c39)
.unwrap();}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("send_frame", image_passer)?;
    cx.export_function("open_device", open_device)?;
    cx.export_function("close_device", close_device)?;
    Ok(())
}

fn open_device(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("s"))
}

fn close_device(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("s"))
}

// PAUSE RENDERING
fn pause_passer(mut cx: FunctionContext) -> JsResult<JsString> {
    GLOBAL_DATA.with(|hid| {
        pause_rendering_and_reset_fb(hid);
    });
    Ok(cx.string("s"))
}

pub fn pause_rendering_and_reset_fb(handle: &hidapi::HidDevice) {
    let _result1 = handle.send_feature_report(&[
        0x03, 0x0d, 0x01, 0x01, 0x78, 0x00, 0xc0, 0x03, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f,
        0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f,
        0x2f, 0xff,
    ]);
    let _result2 = handle.send_feature_report(&[
        0x03, 0x01, 0x64, 0x01, 0x78, 0x00, 0xc0, 0x03, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f,
        0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f,
        0x2f, 0xff,
    ]);
    std::thread::sleep(time::Duration::from_millis(50));
}

// SEND IMAGE
fn image_passer(mut cx: FunctionContext) -> JsResult<JsString> {
    let base_64 = cx.argument::<JsString>(0)?;
    let image = general_purpose::STANDARD
        .decode(base_64.value(&mut cx))
        .unwrap();
    GLOBAL_DATA.with(|hid| {
        send_image(hid, image);
    });
    Ok(cx.string("done"))
}

pub fn send_image(handle: &hidapi::HidDevice, image: Vec<u8>) {
    let device = handle;
    let mut packets_sent = 0;
    for chunk in image.chunks(1016) {
        let mut imgdata: Vec<u8> = Vec::new();
        let signature: u8;
        let chunktrans;
        let chunkand;
        if chunk.len() < 1016 {
            signature = 0x01;
            let eight: u8 = 8;
            let chunklen: u16 = u16::try_from(chunk.len()).unwrap();
            chunktrans = chunklen >> eight & 0xff;
            chunkand = chunklen & 0xff;
            let imgtemp = [
                0x02,
                0x05,
                0x40,
                signature,
                packets_sent,
                0x00,
                shift_verbose_split_u16(chunktrans)[1],
                shift_verbose_split_u16(chunkand)[1],
            ];
            let mut data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            Vec::resize(&mut data, 1016, u8::try_from(0x00).unwrap());
            imgdata.extend(data.to_vec());
        } else {
            signature = 0x00;
            let eight: u8 = 8;
            let chunklen: u16 = u16::try_from(chunk.len()).unwrap();
            chunktrans = chunklen >> eight & 0xff;
            chunkand = chunklen & 0xff;
            let imgtemp = [
                0x02,
                0x05,
                0x40,
                signature,
                packets_sent,
                0x00,
                shift_verbose_split_u16(chunktrans)[1],
                shift_verbose_split_u16(chunkand)[1],
            ];
            let data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            imgdata.extend(data.to_vec());
        }
        let result = device.write(&mut imgdata);
        //     let rehandle: bool;
        //     match result {
        //         _ => rehandle = true,
        //     }
        //     if result.as_ref().unwrap() != &usize::try_from(1024).unwrap()
        //         && signature == 0x01
        //         && !rehandle
        //     {
        //         let mut unfuck_packet = [
        //             0x03,
        //             0x19,
        //             0x40,
        //             signature,
        //             packets_sent,
        //             0x00,
        //             shift_verbose_split_u16(chunktrans)[1],
        //             shift_verbose_split_u16(chunkand)[1],
        //         ]
        //         .to_vec();
        //         unfuck_packet.extend(chunk[0..24].to_vec());
        //         let _result = device.send_feature_report(&mut unfuck_packet);
        //     } else if result.as_ref().is_err() {
        //         std::thread::sleep(std::time::Duration::from_millis(6000));
        //     }
        packets_sent += 1;
    }
}

fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
    let high_byte: u8 = (short_16 >> 8) as u8;
    let low_byte: u8 = (short_16 & 0xff) as u8;

    return [high_byte, low_byte];
}
