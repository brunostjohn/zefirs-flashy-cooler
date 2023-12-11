#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{error::Error, fs::File, io::Write, path::Path};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(debug_assertions)]
const ULTRALIGHT_RESOURCE_CACERT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-resources/cacert.pem")).as_slice();
#[cfg(debug_assertions)]
const ULTRALIGHT_RESOURCE_ICUDT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-resources/icudt67l.dat")).as_slice();
#[cfg(debug_assertions)]
const ULTRALIGHT_BIN_APPCORE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-bin/AppCore.dll")).as_slice();
#[cfg(debug_assertions)]
const ULTRALIGHT_BIN_ULTRALIGHT: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-bin/Ultralight.dll")).as_slice();
#[cfg(debug_assertions)]
const ULTRALIGHT_BIN_ULTRALIGHTCORE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-bin/UltralightCore.dll")).as_slice();
#[cfg(debug_assertions)]
const ULTRALIGHT_BIN_WEBCORE: &[u8] =
    include_bytes!(concat!(env!("OUT_DIR"), "/ul-bin/WebCore.dll")).as_slice();

#[cfg(debug_assertions)]
pub fn extract_ul_resources<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();

    std::fs::create_dir_all(path.join("resources"))?;
    let mut file = File::create(path.join("resources").join("cacert.pem"))?;
    file.write_all(ULTRALIGHT_RESOURCE_CACERT)?;
    let mut file = File::create(path.join("resources").join("icudt67l.dat"))?;
    file.write_all(ULTRALIGHT_RESOURCE_ICUDT)?;

    let mut file = File::create(path.join("AppCore.dll"))?;
    file.write_all(ULTRALIGHT_BIN_APPCORE)?;
    let mut file = File::create(path.join("Ultralight.dll"))?;
    file.write_all(ULTRALIGHT_BIN_ULTRALIGHT)?;
    let mut file = File::create(path.join("UltralightCore.dll"))?;
    file.write_all(ULTRALIGHT_BIN_ULTRALIGHTCORE)?;
    let mut file = File::create(path.join("WebCore.dll"))?;
    file.write_all(ULTRALIGHT_BIN_WEBCORE)?;

    Ok(())
}

pub fn check_for_resources() {
    let mut current_exe = std::env::current_exe().expect("Failed to get current exe path!");
    current_exe.pop();

    let dll_names = [
        "AppCore.dll",
        "Ultralight.dll",
        "UltralightCore.dll",
        "WebCore.dll",
    ];

    for dll_name in dll_names.iter() {
        if !current_exe.join(dll_name).exists() {
            panic!("Missing Ultralight resource: {}", dll_name);
        }
    }

    let resource_names = ["cacert.pem", "icudt67l.dat"];

    for resource_name in resource_names.iter() {
        if !current_exe.join("resources").join(resource_name).exists() {
            panic!("Missing Ultralight resource: {}", resource_name);
        }
    }
}

#[cfg(debug_assertions)]
pub fn delete_ul_resources<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn Error>> {
    let path = path.as_ref();

    std::fs::remove_file(path.join("resources").join("cacert.pem"))?;
    std::fs::remove_file(path.join("resources").join("icudt67l.dat"))?;

    std::fs::remove_file(path.join("AppCore.dll"))?;
    std::fs::remove_file(path.join("Ultralight.dll"))?;
    std::fs::remove_file(path.join("UltralightCore.dll"))?;
    std::fs::remove_file(path.join("WebCore.dll"))?;

    Ok(())
}
