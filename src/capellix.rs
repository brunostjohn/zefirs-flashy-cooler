pub mod capellix {
    use base64::{engine::general_purpose, Engine as _};
    use neon::prelude::*;
    use neon::result::JsResult;
    use neon::types::JsString;
    use std::convert::TryFrom;
    extern crate hidapi;

    fn shift_verbose_split_u16(short_16: u16) -> [u8; 2] {
        let high_byte: u8 = (short_16 >> 8) as u8;
        let low_byte: u8 = (short_16 & 0xff) as u8;

        return [high_byte, low_byte];
    }

    pub fn send_image(mut cx: FunctionContext, hid: hidapi::HidApi) -> JsResult<JsString> {
        let base_64 = cx.argument::<JsString>(0)?;
        let image = general_purpose::STANDARD
            .decode(base_64.value(&mut cx))
            .unwrap();
        let mut packets_sent = 0;
        let api = hidapi::HidApi::new_without_enumerate().unwrap();
        let (vid, pid) = (0x1b1c, 0x0c39);
        let device = api.open(vid, pid).unwrap();
        for chunk in image.chunks(1016) {
            let mut imgdata: Vec<u8> = Vec::new();
            let signature: u8;
            if chunk.len() < 1016 {
                signature = 0x01;
                let eight: u8 = 8;
                let chunklen: u16 = u16::try_from(chunk.len()).unwrap();
                let chunktrans = chunklen >> eight & 0xff;
                let chunkand = chunklen & 0xff;
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
                let chunktrans = chunklen >> eight & 0xff;
                let chunkand = chunklen & 0xff;
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
            device.write(&mut imgdata).unwrap();
            packets_sent += 1;
        }
        Ok(cx.string("done"))
    }
}
