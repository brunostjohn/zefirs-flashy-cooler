extern crate hidapi;
use base64::{engine::general_purpose, Engine as _};
use hidapi::HidDevice;
use neon::prelude::*;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::sync::Mutex;
use std::time::SystemTime;

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1b1c, 0x0c39)
.unwrap();}

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
    GLOBAL_DATA.with(|hid| {
        hid.send_feature_report(&[0x03, 0x1d, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00])
            .expect("Failed to say hello!");
        hid.send_feature_report(&[0x03, 0x19, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00])
            .expect("Failed to issue request 0x19!");
        hid.send_feature_report(&[0x03, 0x20, 0x00, 0x19, 0x79, 0xe7, 0x32, 0x2e])
            .expect("Failed to issue request 0x20!");
        hid.send_feature_report(&[0x03, 0x0b, 0x40, 0x01, 0x79, 0xe7, 0x32, 0x2e])
            .expect("Failed to set interface!");
    });
    std::thread::sleep(std::time::Duration::from_millis(5));
    Ok(cx.string("s"))
}

fn close_device(mut cx: FunctionContext) -> JsResult<JsString> {
    std::thread::sleep(std::time::Duration::from_millis(5));
    GLOBAL_DATA.with(|hid| {
        hid.send_feature_report(&[0x03, 0x1e, 0x40, 0x01, 0x43, 0x00, 0x69, 0x00])
            .expect("Failed to close LCD!");
        drop(hid);
    });
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
    if !failed && dur > std::time::Duration::from_secs(25) {
        please_unfuck = true;
        timevec.remove(0);
        timevec.push(SystemTime::now());
    }
    let image = general_purpose::STANDARD
        .decode(base_64.value(&mut cx))
        .unwrap();
    GLOBAL_DATA.with(|hid| {
        send_image(hid, image, please_unfuck);
    });
    Ok(cx.string("done"))
}

pub fn send_image(handle: &hidapi::HidDevice, image: Vec<u8>, please_unfuck: bool) {
    let device = handle;
    let mut packets_sent = 0;
    let mut last_image: Vec<u8> = Vec::new();
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
                shift_verbose_split_u16(chunkand)[1],
                shift_verbose_split_u16(chunktrans)[1],
            ];
            let data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            imgdata.extend(data.to_vec());
            imgdata.append(&mut last_image[(usize::try_from(chunklen).unwrap())..].to_vec());
        } else {
            signature = 0x00;
            let imgtemp = [0x02, 0x05, 0x40, signature, packets_sent, 0x00, 0xf8, 0x03];
            let data = Vec::try_from(chunk).unwrap();
            imgdata.append(&mut imgtemp.to_vec());
            imgdata.extend(data.to_vec());
            last_image = data.to_vec();
        }
        let unhandled_result = device.write(&mut imgdata);
        let result;
        match unhandled_result {
            Ok(we_did_it) => result = we_did_it,
            Err(_nope) => result = 1023,
        }
        if signature == 0x01 && result != usize::try_from(1024).unwrap()
            || signature == 0x01 && please_unfuck
        {
            let chunklen: u16 = u16::try_from(chunk.len()).unwrap();
            let chunktrans_temp = chunklen >> 8 & 0xff;
            let chunkand_temp = chunklen & 0xff;
            let mut unfuck_packet = [
                0x03,
                0x19,
                0x40,
                signature,
                packets_sent,
                0x00,
                shift_verbose_split_u16(chunkand_temp)[1],
                shift_verbose_split_u16(chunktrans_temp)[1],
            ]
            .to_vec();
            let _result = device.send_feature_report(&mut unfuck_packet);
        }
        packets_sent += 1;
    }
}

fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
    let high_byte: u8 = (short_16 >> 8) as u8;
    let low_byte: u8 = (short_16 & 0xff) as u8;

    return [high_byte, low_byte];
}
