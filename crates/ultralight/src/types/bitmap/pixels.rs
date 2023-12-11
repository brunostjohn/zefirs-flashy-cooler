use super::r#impl::Bitmap;
use std::ops::{Deref, DerefMut};

pub(crate) struct PixelsGuard<'a> {
    lock: &'a mut Bitmap,
    pixels: &'a mut [u8],
}

impl<'a> PixelsGuard<'a> {
    pub(crate) unsafe fn new(lock: &'a mut Bitmap, pixels: &'a mut [u8]) -> PixelsGuard<'a> {
        PixelsGuard { lock, pixels }
    }
}

impl Deref for PixelsGuard<'_> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.pixels
    }
}

impl DerefMut for PixelsGuard<'_> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.pixels
    }
}

impl Drop for PixelsGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            self.lock.raw_unlock_pixels();
        }
    }
}
