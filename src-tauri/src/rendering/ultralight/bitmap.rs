use std::{
    convert::{TryFrom, TryInto},
    ffi::{c_void, CString},
    ops::{Deref, DerefMut},
    path::Path,
    slice,
};

use ul_sys::*;

#[derive(Debug, thiserror::Error)]
pub enum BitmapError {
    #[error("Creation of bitmap failed because Ultralight returned a null pointer")]
    NullReference,
    #[error(
        "Creation of bitmap failed because it required {required} bytes, but got {got} bytes only"
    )]
    PixelBufferSizeMismatch { got: usize, required: usize },
    #[error("Tried to swap red and blue channels on an unsupported format")]
    UnsupportedOperationForPixelFormat,
    #[error("Could not write bitmap to PNG successfully")]
    FailedPngWrite,
    #[error("Could not create bitmap because its empty")]
    EmptyBitmap,
}

type BitmapResult<T> = std::result::Result<T, BitmapError>;

#[derive(Debug, Clone, Copy)]
pub enum BitmapFormat {
    A8Unorm = ULBitmapFormat_kBitmapFormat_A8_UNORM as isize,
    Bgra8UnormSrgb = ULBitmapFormat_kBitmapFormat_BGRA8_UNORM_SRGB as isize,
}

impl TryFrom<ULBitmapFormat> for BitmapFormat {
    type Error = ();

    #[allow(non_upper_case_globals)]
    fn try_from(format: ULBitmapFormat) -> Result<Self, Self::Error> {
        match format {
            ULBitmapFormat_kBitmapFormat_A8_UNORM => Ok(BitmapFormat::A8Unorm),
            ULBitmapFormat_kBitmapFormat_BGRA8_UNORM_SRGB => Ok(BitmapFormat::Bgra8UnormSrgb),
            _ => Err(()),
        }
    }
}

impl BitmapFormat {
    pub fn bytes_per_pixel(&self) -> u32 {
        match self {
            BitmapFormat::A8Unorm => 1,
            BitmapFormat::Bgra8UnormSrgb => 4,
        }
    }
}

pub struct PixelsGuard<'a> {
    lock: &'a mut Bitmap,
    pixels: &'a mut [u8],
}

impl<'a> PixelsGuard<'a> {
    unsafe fn new(lock: &'a mut Bitmap, pixels: &'a mut [u8]) -> PixelsGuard<'a> {
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
        &mut self.pixels
    }
}

impl Drop for PixelsGuard<'_> {
    fn drop(&mut self) {
        unsafe {
            self.lock.raw_unlock_pixels();
        }
    }
}

pub struct Bitmap {
    internal: ULBitmap,
    need_to_destroy: bool,
}

impl Bitmap {
    pub(crate) unsafe fn from_raw(raw: ULBitmap) -> Option<Self> {
        if raw.is_null() {
            return None;
        }

        Some(Bitmap {
            internal: raw,
            need_to_destroy: false,
        })
    }

    pub fn create_empty() -> BitmapResult<Self> {
        let internal = unsafe { ulCreateEmptyBitmap() };

        if internal.is_null() {
            Err(BitmapError::NullReference)
        } else {
            Ok(Self {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn create(width: usize, height: usize, format: BitmapFormat) -> BitmapResult<Self> {
        let internal = unsafe { ulCreateBitmap(width as u32, height as u32, format as u32) };
        if internal.is_null() {
            Err(BitmapError::NullReference)
        } else {
            Ok(Self {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn create_from_pixels(
        width: u32,
        height: u32,
        format: BitmapFormat,
        pixels: &[u8],
    ) -> BitmapResult<Self> {
        let row_bytes = width * format.bytes_per_pixel();
        let bytes_size = (height * row_bytes) as usize;
        if pixels.len() != bytes_size {
            return Err(BitmapError::PixelBufferSizeMismatch {
                got: pixels.len(),
                required: bytes_size,
            });
        }

        let internal = unsafe {
            ulCreateBitmapFromPixels(
                width,
                height,
                format as u32,
                row_bytes,
                pixels.as_ptr() as *const c_void,
                pixels.len() as u32,
                true,
            )
        };
        if internal.is_null() {
            Err(BitmapError::NullReference)
        } else {
            Ok(Self {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn copy(&self) -> BitmapResult<Self> {
        let internal = unsafe { ulCreateBitmapFromCopy(self.internal) };

        if internal.is_null() {
            Err(BitmapError::NullReference)
        } else {
            Ok(Self {
                internal,
                need_to_destroy: true,
            })
        }
    }
}

impl Bitmap {
    pub fn width(&self) -> u32 {
        unsafe { ulBitmapGetWidth(self.internal) }
    }

    pub fn height(&self) -> u32 {
        unsafe { ulBitmapGetHeight(self.internal) }
    }

    pub fn format(&self) -> BitmapFormat {
        unsafe { ulBitmapGetFormat(self.internal) }
            .try_into()
            .unwrap()
    }

    pub fn bpp(&self) -> u32 {
        unsafe { ulBitmapGetBpp(self.internal) }
    }

    pub fn row_bytes(&self) -> u32 {
        unsafe { ulBitmapGetRowBytes(self.internal) }
    }

    pub fn bytes_size(&self) -> u32 {
        unsafe { ulBitmapGetSize(self.internal.into()) }
    }

    pub fn lock_pixels(&mut self) -> Option<PixelsGuard> {
        let (raw_pixels, size) = unsafe {
            ulBitmapLockPixels(self.internal);
            (
                ulBitmapRawPixels(self.internal),
                ulBitmapGetSize(self.internal),
            )
        };

        if raw_pixels.is_null() {
            return None;
        }

        unsafe {
            let data = slice::from_raw_parts_mut(raw_pixels as _, size as usize);
            Some(PixelsGuard::new(self, data))
        }
    }

    pub(crate) unsafe fn raw_unlock_pixels(&mut self) {
        ulBitmapUnlockPixels(self.internal);
    }

    pub fn is_empty(&self) -> bool {
        unsafe { ulBitmapIsEmpty(self.internal) }
    }

    pub fn erase(&self) {
        unsafe { ulBitmapErase(self.internal) }
    }

    pub fn write_to_png<P: AsRef<Path>>(&self, path: P) -> BitmapResult<()> {
        let c_path = CString::new(path.as_ref().to_str().unwrap()).unwrap();
        let result = unsafe { ulBitmapWritePNG(self.internal, c_path.as_ptr()) };
        if result {
            Ok(())
        } else {
            Err(BitmapError::FailedPngWrite)
        }
    }

    pub fn swap_red_blue_channels(&self) -> BitmapResult<()> {
        if let BitmapFormat::Bgra8UnormSrgb = self.format() {
            unsafe { ulBitmapSwapRedBlueChannels(self.internal) }
            Ok(())
        } else {
            Err(BitmapError::UnsupportedOperationForPixelFormat)
        }
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        if self.need_to_destroy {
            unsafe { ulDestroyBitmap(self.internal) };
        }
    }
}

pub struct OwnedBitmap {
    width: u32,
    height: u32,
    format: BitmapFormat,
    bpp: u32,
    row_bytes: u32,
    bytes_size: u64,
    pixels: Option<Vec<u8>>,
    is_empty: bool,
}

impl OwnedBitmap {
    pub fn from_bitmap(bitmap: &mut Bitmap) -> Option<Self> {
        let width = bitmap.width();
        let height = bitmap.height();
        let format = bitmap.format();
        let bpp = bitmap.bpp();
        let row_bytes = bitmap.row_bytes();
        let bytes_size = bitmap.bytes_size();
        let is_empty = bitmap.is_empty();

        let pixels = bitmap.lock_pixels().map(|v| v.to_vec());

        Some(Self {
            width,
            height,
            format,
            bpp,
            row_bytes,
            bytes_size: bytes_size.into(),
            pixels,
            is_empty,
        })
    }

    pub fn to_bitmap(&self) -> BitmapResult<Bitmap> {
        if let Some(pixels) = self.pixels.as_ref() {
            Bitmap::create_from_pixels(self.width, self.height, self.format, pixels.as_slice())
        } else {
            Err(BitmapError::EmptyBitmap)
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn format(&self) -> BitmapFormat {
        self.format
    }

    pub fn bpp(&self) -> u32 {
        self.bpp
    }

    pub fn row_bytes(&self) -> u32 {
        self.row_bytes
    }

    pub fn bytes_size(&self) -> u64 {
        self.bytes_size
    }

    pub fn pixels(&self) -> Option<&[u8]> {
        self.pixels.as_deref()
    }

    pub fn pixels_mut(&mut self) -> Option<&mut [u8]> {
        self.pixels.as_deref_mut()
    }

    pub fn is_empty(&self) -> bool {
        self.is_empty
    }
}
