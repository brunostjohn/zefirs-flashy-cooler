use crate::constants::constants::*;
mod constants;
use base64::{engine::general_purpose, Engine as _};
use hidapi_rusb::{HidApi, HidDevice};
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::sync::{Mutex, MutexGuard};
use std::time::SystemTime;
extern crate hidapi_rusb;

static HIDAPI: Lazy<Mutex<HidApi>> = Lazy::new(|| {
    let api = hidapi_rusb::HidApi::new().expect("failed to create api!");
    let muta = Mutex::new(api);
    muta
});

static HID: Lazy<Mutex<Vec<HidDevice>>> = Lazy::new(|| {
    let device = HIDAPI.lock().unwrap().open(VENDOR_ID, PRODUCT_ID).unwrap();
    let mut ve = Vec::new();
    ve.push(device);
    let muta = Mutex::new(ve);
    muta
});

static UNFUCK_TIME: Lazy<Mutex<Vec<SystemTime>>> = Lazy::new(|| {
    let mut cur_time = Vec::new();
    cur_time.push(SystemTime::now());
    Mutex::new(cur_time)
});

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("send_frame", image_passer)?;
    cx.export_function("open_device", open_device)?;
    cx.export_function("close_device", close_device)?;
    Ok(())
}

fn open_device(mut cx: FunctionContext) -> JsResult<JsString> {
    // let hidt = HID.lock().unwrap();
    // let hid = hidt.get(0).unwrap();
    // hid.send_feature_report(&[
    //     CONTROL_REQUEST,
    //     DEVICE_STAT,
    //     0x01,
    //     0x00,
    //     0x00,
    //     0x00,
    //     0x00,
    //     0x00,
    // ])
    // .expect("Failed to say hello!");
    // hid.send_feature_report(&[
    //     CONTROL_REQUEST,
    //     DEVICE_ALIVE,
    //     0x00,
    //     0x00,
    //     0x00,
    //     0x00,
    //     0x00,
    //     0x00,
    // ])
    // .expect("Failed to issue request 0x19!");
    // hid.send_feature_report(&[CONTROL_REQUEST, 0x20, 0x00, 0x19, 0x79, 0xe7, 0x32, 0x2e])
    //     .expect("Failed to issue request 0x20!");
    // hid.send_feature_report(&[
    //     CONTROL_REQUEST,
    //     SET_INTERFACE,
    //     0x40,
    //     0x01,
    //     0x79,
    //     0xe7,
    //     0x32,
    //     0x2e,
    // ])
    // .expect("Failed to set interface!");
    // std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(cx.string("s"))
}

fn close_device(mut cx: FunctionContext) -> JsResult<JsString> {
    // std::thread::sleep(std::time::Duration::from_millis(5));
    // let hidt = HID.lock().unwrap();
    // let hid = hidt.get(0).unwrap();
    // hid.send_feature_report(&[CONTROL_REQUEST, 0x1e, 0x40, 0x01, 0x43, 0x00, 0x69, 0x00])
    //     .expect("Failed to close LCD!");
    Ok(cx.string("s"))
}

// SEND IMAGE
fn image_passer(mut cx: FunctionContext) -> JsResult<JsString> {
    let base_64 = cx.argument::<JsString>(0)?;
    let mut timevec = UNFUCK_TIME.lock().unwrap();
    let elapsed = timevec.get(0).unwrap().elapsed();
    let mut failed = false;
    let mut dur = std::time::Duration::from_secs(25);
    let mut please_unfuck = false;
    match elapsed {
        Ok(ok) => dur = ok,
        Err(_error) => failed = true,
    }
    if !failed && (dur > std::time::Duration::from_secs(25)) {
        please_unfuck = true;
        timevec.remove(0);
        timevec.push(SystemTime::now());
    }
    let image = general_purpose::STANDARD
        .decode(base_64.value(&mut cx))
        .unwrap();
    let mut dev = HID.lock().unwrap();
    let retval = send_image(&dev, image, please_unfuck);
    if !retval {
        std::thread::sleep(std::time::Duration::from_millis(7000));
        dev.remove(0);
        dev.push(HIDAPI.lock().unwrap().open(VENDOR_ID, PRODUCT_ID).unwrap());
    }
    Ok(cx.string("done"))
}

pub fn send_image(
    handle: &MutexGuard<Vec<HidDevice>>,
    image: Vec<u8>,
    please_unfuck: bool,
) -> bool {
    let device = handle.get(0).unwrap();
    let mut packets_sent = 0;
    let mut last_image: Vec<u8> = Vec::new();
    let mut retval = false;
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
                IMG_TX,
                0x09,
                0x64,
                signature,
                shift_verbose_split_u16(chunkand)[1],
                shift_verbose_split_u16(chunktrans)[1],
                packets_sent,
                0x00,
            ];
            let data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            imgdata.extend(data.to_vec());
            imgdata.append(&mut last_image[(usize::try_from(chunklen).unwrap())..].to_vec());
        } else {
            signature = 0x00;
            let imgtemp = [
                IMG_TX,
                0x09,
                0x65,
                signature,
                0xf8,
                0x03,
                packets_sent,
                0x00,
            ];
            let data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            imgdata.extend(data.to_vec());
            last_image = data.to_vec();
        }
        let unhandled_result = device.write(&mut imgdata);
        match unhandled_result {
            Ok(_we_did_it) => retval = true,
            Err(_nope) => retval = false,
        }
        if signature == 0x01 && please_unfuck {
            // let chunklen: u16 = u16::try_from(chunk.len()).unwrap();
            // let chunktrans_temp = chunklen >> 8 & 0xff;
            // let chunkand_temp = chunklen & 0xff;
            // let mut unfuck_packet = [
            //     CONTROL_REQUEST,
            //     DEVICE_ALIVE,
            //     0x40,
            //     signature,
            //     packets_sent,
            //     0x00,
            //     shift_verbose_split_u16(chunkand_temp)[1],
            //     shift_verbose_split_u16(chunktrans_temp)[1],
            // ]
            // .to_vec();
            // let _result = device.send_feature_report(&mut unfuck_packet);
        }
        packets_sent += 1;
    }
    retval
}

fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
    let high_byte: u8 = (short_16 >> 8) as u8;
    let low_byte: u8 = (short_16 & 0xff) as u8;

    return [high_byte, low_byte];
}
