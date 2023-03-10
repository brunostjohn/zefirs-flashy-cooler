use hidapi::HidApi;
use neon::prelude::*;
use std::cell::RefCell;

mod capellix;
use crate::capellix::capellix::send_image;

fn send_img_pass(mut cx: FunctionContext) -> JsResult<JsString> {
    // this should pass the hidapi object and device handle
    Ok(cx.string("done"))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("capellix_send_frame", send_img_pass)?;
    Ok(())
}
