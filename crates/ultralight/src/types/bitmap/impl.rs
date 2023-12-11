use std::path::Path;

use ultralight_sys::{
    ulBitmapErase, ulBitmapGetBpp, ulBitmapGetFormat, ulBitmapGetHeight, ulBitmapGetRowBytes,
    ulBitmapGetSize, ulBitmapGetWidth, ulBitmapIsEmpty, ulBitmapLockPixels, ulBitmapRawPixels,
    ulBitmapSwapRedBlueChannels, ulBitmapUnlockPixels, ulBitmapWritePNG, ULBitmap,
};

use crate::{error::ULError, string::ULString, ULResult};

use super::{format::BitmapFormat, pixels::PixelsGuard};

#[derive(Debug)]
pub enum Bitmap {
    Borrowed {
        internal: ultralight_sys::ULBitmap,
        need_to_destroy: bool,
    },
    Owned {
        width: u32,
        height: u32,
        format: BitmapFormat,
        bpp: u32,
        row_bytes: u32,
        bytes_size: u64,
        pixels: Option<Vec<u8>>,
        is_empty: bool,
    },
}

impl Bitmap {
    pub unsafe fn from_raw(raw: ULBitmap) -> Option<Self> {
        if raw.is_null() {
            None
        } else {
            Some(Self::Borrowed {
                internal: raw,
                need_to_destroy: false,
            })
        }
    }

    pub fn create_empty_borrowed() -> ULResult<Self> {
        let internal = unsafe { ultralight_sys::ulCreateEmptyBitmap() };
        if internal.is_null() {
            Err(ULError::BitmapNullReference)
        } else {
            Ok(Self::Borrowed {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn create_borrowed(width: u32, height: u32, format: BitmapFormat) -> ULResult<Self> {
        let internal = unsafe { ultralight_sys::ulCreateBitmap(width, height, format as _) };
        if internal.is_null() {
            Err(ULError::BitmapNullReference)
        } else {
            Ok(Self::Borrowed {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn create_borrowed_from_pixels(
        width: u32,
        height: u32,
        format: BitmapFormat,
        row_bytes: u32,
        pixels: &[u8],
    ) -> ULResult<Self> {
        let internal = unsafe {
            ultralight_sys::ulCreateBitmapFromPixels(
                width,
                height,
                format as _,
                row_bytes,
                pixels.as_ptr() as _,
                pixels.len(),
                true,
            )
        };
        if internal.is_null() {
            Err(ULError::BitmapNullReference)
        } else {
            Ok(Self::Borrowed {
                internal,
                need_to_destroy: true,
            })
        }
    }

    pub fn copy_borrowed(&self) -> ULResult<Self> {
        match self {
            Self::Borrowed { internal, .. } => {
                let internal = unsafe { ultralight_sys::ulCreateBitmapFromCopy(*internal) };
                if internal.is_null() {
                    Err(ULError::BitmapNullReference)
                } else {
                    Ok(Self::Borrowed {
                        internal,
                        need_to_destroy: true,
                    })
                }
            }
            Self::Owned { .. } => Err(ULError::BitmapUnsupportedOperationForOwnedBitmap),
        }
    }

    pub fn width(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetWidth(*internal) },
            Self::Owned { width, .. } => *width,
        }
    }

    pub fn height(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetHeight(*internal) },
            Self::Owned { height, .. } => *height,
        }
    }

    pub fn format(&self) -> ULResult<BitmapFormat> {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetFormat(*internal) }.try_into(),
            Self::Owned { format, .. } => Ok(*format),
        }
    }

    pub fn bpp(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetBpp(*internal) },
            Self::Owned { bpp, .. } => *bpp,
        }
    }

    pub fn row_bytes(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetRowBytes(*internal) },
            Self::Owned { row_bytes, .. } => *row_bytes,
        }
    }

    pub fn bytes_size(&self) -> u64 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetSize(*internal) }
                .try_into()
                .expect("Failed to convert size to u64"),
            Self::Owned { bytes_size, .. } => *bytes_size,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapIsEmpty(*internal) },
            Self::Owned { is_empty, .. } => *is_empty,
        }
    }

    pub fn lock_pixels(&mut self) -> Option<PixelsGuard> {
        match self {
            Self::Borrowed { internal, .. } => {
                let (raw_pixels, size) = unsafe {
                    ulBitmapLockPixels(*internal);
                    (ulBitmapRawPixels(*internal), ulBitmapGetSize(*internal))
                };
                if raw_pixels.is_null() {
                    None
                } else {
                    let pixels = unsafe { std::slice::from_raw_parts_mut(raw_pixels as _, size) };
                    Some(unsafe { PixelsGuard::new(self, pixels) })
                }
            }
            Self::Owned { .. } => None,
        }
    }

    pub fn raw_unlock_pixels(&mut self) {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapUnlockPixels(*internal) },
            Self::Owned { .. } => {}
        }
    }

    pub fn erase(&self) {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapErase(*internal) },
            Self::Owned { .. } => {}
        }
    }

    pub fn write_to_png<P: AsRef<Path>>(&self, path: P) -> ULResult<()> {
        let path = path.as_ref();
        let path = path.to_str().ok_or(ULError::BitmapPNGInvalidPath)?;
        let path = ULString::from(path);
        match self {
            Self::Borrowed { internal, .. } => unsafe {
                ulBitmapWritePNG(*internal, *path as _);
            },
            Self::Owned { .. } => {}
        }
        Ok(())
    }

    pub fn swap_red_blue_channels(&self) -> ULResult<()> {
        match self {
            Self::Borrowed { internal, .. } => {
                if let BitmapFormat::Bgra8UnormSrgb = self.format()? {
                    unsafe { ulBitmapSwapRedBlueChannels(*internal) };
                    Ok(())
                } else {
                    Err(ULError::BitmapUnsupportedOperationForPixelFormat)
                }
            }
            Self::Owned { .. } => Err(ULError::BitmapUnsupportedOperationForPixelFormat),
        }
    }

    pub fn to_owned(mut self) -> ULResult<Self> {
        match self {
            Self::Borrowed { .. } => {
                let width = self.width();
                let height = self.height();
                let format = self.format()?;
                let bpp = self.bpp();
                let row_bytes = self.row_bytes();
                let bytes_size = self.bytes_size();
                let is_empty = self.is_empty();
                let pixels = self.lock_pixels().ok_or(ULError::BitmapNullReference)?;
                let pixels = pixels.to_vec();

                Ok(Self::Owned {
                    width,
                    height,
                    format,
                    bpp,
                    row_bytes,
                    bytes_size,
                    pixels: Some(pixels),
                    is_empty,
                })
            }
            Self::Owned { .. } => Ok(self),
        }
    }

    pub fn from_owned(mut self) -> ULResult<Self> {
        todo!();
    }
}

impl Drop for Bitmap {
    fn drop(&mut self) {
        match self {
            Self::Borrowed {
                internal,
                need_to_destroy,
            } => {
                if *need_to_destroy {
                    unsafe {
                        ultralight_sys::ulDestroyBitmap(*internal);
                    }
                }
            }
            Self::Owned { .. } => {}
        }
    }
}
