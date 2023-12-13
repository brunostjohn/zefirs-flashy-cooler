use crate::traits::{device_creator::DeviceCreator, display_cooler::DisplayCooler};

use super::{device::CorsairH150i, *};

#[test]
fn creates_a_device() {
    tokio::runtime::Builder::new_multi_thread()
        .build()
        .unwrap()
        .block_on(async {
            let device = CorsairH150i::create_device().await.unwrap();
        });
}

#[test]
fn initialises_a_device() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let mut device = CorsairH150i::create_device().await.unwrap();
            device.initialise().await.unwrap();
        });
}

#[test]
fn sends_red_rgba_image() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let mut device = CorsairH150i::create_device().await.unwrap();
            device.initialise().await.unwrap();

            let image: Vec<u8> = vec![&[255u8, 0u8, 0u8, 255u8]; 480 * 480 * 4]
                .into_iter()
                .flatten()
                .copied()
                .collect();
            device.send_image(&image).await.unwrap();
        });
}

#[test]
fn sends_green_rgba_image_and_closes() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_time()
        .build()
        .unwrap()
        .block_on(async {
            let mut device = CorsairH150i::create_device().await.unwrap();
            device.initialise().await.unwrap();

            let image: Vec<u8> = vec![&[0u8, 255u8, 0u8, 255u8]; 480 * 480 * 4]
                .into_iter()
                .flatten()
                .copied()
                .collect();
            device.send_image(&image).await.unwrap();

            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

            device.close().await.unwrap();
        });
}
