use crate::traits::{device_creator::DeviceCreator, display_cooler::DisplayCooler};
use hidapi::{HidApi, HidDevice};
use tokio::{
    task,
    time::{self, Duration},
};
use turbojpeg::{Compressor, Image, OutputBuf};

use super::constants::{CONTROL_REQUEST, DEVICE_ALIVE, DEVICE_STAT, IMG_TX, SET_INTERFACE};

pub struct CorsairH150i<'a> {
    #[allow(unused)]
    pub(crate) api: HidApi,
    pub(crate) device: HidDevice,
    pub(crate) compressor: Compressor,
    pub(crate) image_frame: Image<&'a [u8]>,
    pub(crate) compression_buffer: OutputBuf<'a>,
    pub(crate) packet: Vec<u8>,
}

unsafe impl Send for CorsairH150i<'_> {}
unsafe impl Sync for CorsairH150i<'_> {}

impl<'a> CorsairH150i<'a> {
    pub(crate) async fn send_feature_report(&mut self, data: &[u8]) -> anyhow::Result<()> {
        task::block_in_place(|| -> anyhow::Result<()> {
            self.device.send_feature_report(data)?;
            Ok(())
        })
    }
}

impl<'a> DisplayCooler for CorsairH150i<'a> {
    async fn initialise(&mut self) -> anyhow::Result<()> {
        self.send_feature_report(&[
            CONTROL_REQUEST,
            DEVICE_STAT,
            0x01,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ])
        .await?;

        self.send_feature_report(&[
            CONTROL_REQUEST,
            DEVICE_ALIVE,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
        ])
        .await?;

        self.send_feature_report(&[CONTROL_REQUEST, 0x20, 0x00, 0x19, 0x79, 0xe7, 0x32, 0x2e])
            .await?;

        self.send_feature_report(&[
            CONTROL_REQUEST,
            SET_INTERFACE,
            0x40,
            0x01,
            0x79,
            0xe7,
            0x32,
            0x2e,
        ])
        .await?;

        time::sleep(Duration::from_millis(5)).await;

        Ok(())
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        time::sleep(Duration::from_millis(5)).await;

        self.send_feature_report(&[CONTROL_REQUEST, 0x1e, 0x40, 0x01, 0x43, 0x00, 0x69, 0x00])
            .await?;

        Ok(())
    }

    async fn reopen(self) -> anyhow::Result<impl DisplayCooler + DeviceCreator> {
        Self::create_device().await
    }

    async fn send_image(&mut self, image: &[u8]) -> anyhow::Result<()> {
        let mut last_image = [0u8; 1016].as_slice();
        let mut image_struct = self.image_frame;
        image_struct.pixels = image;

        self.compressor
            .compress(image_struct, &mut self.compression_buffer)
            .unwrap();

        if self.compression_buffer.is_empty() {
            self.compressor = turbojpeg::Compressor::new().unwrap();
            self.compressor.set_quality(95);
            self.compressor.set_subsamp(turbojpeg::Subsamp::Sub2x2);
        }

        let handle = &self.device;

        for (packets_sent, chunk) in self.compression_buffer.chunks(1016).enumerate() {
            let chunk_length = chunk.len() as u16;

            self.packet.extend([IMG_TX, 0x05, 0x40].iter());

            if chunk_length == 1016 {
                self.packet
                    .extend([0x00, packets_sent.try_into().unwrap(), 0x00, 0xf8, 0x03].iter());
            } else {
                self.packet.extend(
                    [
                        0x01,
                        packets_sent.try_into().unwrap(),
                        0x00,
                        (chunk_length & 0xff) as u8,
                        (chunk_length >> 8 & 0xff) as u8,
                    ]
                    .iter(),
                );
            }

            self.packet.extend(chunk.iter());

            if chunk_length < 1016 {
                self.packet
                    .extend(&mut last_image[chunk_length as usize..].iter());
            }

            last_image = chunk;

            handle.write(&self.packet)?;

            // if chunk_length < 1016 && self.unfuck_counter.check_time() {
            //     let unfuck_packet = [
            //         CONTROL_REQUEST,
            //         DEVICE_ALIVE,
            //         0x40,
            //         0x01,
            //         packets_sent.try_into().unwrap(),
            //         0x00,
            //         (chunk_length & 0xff) as u8,
            //         (chunk_length >> 8 & 0xff) as u8,
            //     ];

            //     handle.send_feature_report(&unfuck_packet)?;
            // }

            self.packet.clear();
        }

        Ok(())
    }
}
