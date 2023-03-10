pub mod capellix {
    use core::time;
    use neon::prelude::*;
    use std::convert::TryFrom;
    use std::sync::atomic::AtomicBool;
    extern crate hidapi;

    static RENDER_FLAG: AtomicBool = AtomicBool::new(true);

    fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
        let high_byte: u8 = (short_16 >> 8) as u8;
        let low_byte: u8 = (short_16 & 0xff) as u8;

        return [high_byte, low_byte];
    }

    pub fn pause_rendering_and_reset_fb(mut cx: FunctionContext) -> JsResult<JsString> {
        RENDER_FLAG.store(false, std::sync::atomic::Ordering::SeqCst);
        Ok(cx.string("done"))
    }

    pub fn send_image(handle: &hidapi::HidDevice, image: Vec<u8>) {
        let device = handle;
        if RENDER_FLAG.load(std::sync::atomic::Ordering::Relaxed) == true {
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
                if device.write(&mut imgdata).unwrap() != 1024 && signature == 0x01 {
                    let mut unfuck_packet = [
                        0x03,
                        0x19,
                        0x40,
                        signature,
                        packets_sent,
                        0x00,
                        shift_verbose_split_u16(chunktrans)[1],
                        shift_verbose_split_u16(chunkand)[1],
                    ]
                    .to_vec();
                    unfuck_packet.extend(chunk[0..24].to_vec());
                    let _result = device.send_feature_report(&mut unfuck_packet);
                }
                packets_sent += 1;
            }
        } else {
            let _result1 = device.send_feature_report(&[
                0x03, 0x0d, 0x01, 0x01, 0x78, 0x00, 0xc0, 0x03, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f,
                0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff,
                0x2f, 0x2f, 0x2f, 0xff,
            ]);
            let _result2 = device.send_feature_report(&[
                0x03, 0x01, 0x64, 0x01, 0x78, 0x00, 0xc0, 0x03, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f,
                0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff, 0x2f, 0x2f, 0x2f, 0xff,
                0x2f, 0x2f, 0x2f, 0xff,
            ]);
            std::thread::sleep(time::Duration::from_millis(100));
            RENDER_FLAG.store(true, std::sync::atomic::Ordering::SeqCst);
        }
    }
}
