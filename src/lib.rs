use base64::{engine::general_purpose, Engine as _};
use hidapi::HidDevice;
use neon::prelude::*;

mod capellix;
use crate::capellix::capellix::pause_rendering_and_reset_fb;
use crate::capellix::capellix::send_image;
extern crate hidapi;

thread_local! {static GLOBAL_DATA: HidDevice = hidapi::HidApi::new_without_enumerate()
.unwrap()
.open(0x1b1c, 0x0c39)
.unwrap();}

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

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("capellix_send_frame", image_passer)?;
    cx.export_function("reset_fb", pause_rendering_and_reset_fb)?;
    Ok(())
}
