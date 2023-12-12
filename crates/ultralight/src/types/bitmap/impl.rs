use std::path::Path;

use ultralight_sys::{
    ulBitmapErase, ulBitmapGetBpp, ulBitmapGetFormat, ulBitmapGetHeight, ulBitmapGetRowBytes,
    ulBitmapGetSize, ulBitmapGetWidth, ulBitmapIsEmpty, ulBitmapLockPixels, ulBitmapRawPixels,
    ulBitmapSwapRedBlueChannels, ulBitmapUnlockPixels, ulBitmapWritePNG, ULBitmap,
};

use crate::{error::ULError, string::ULString, ULResult};

use super::{format::BitmapFormat, pixels::PixelsGuard, LockablePixels};

/// Wrapper around Ultralight's [ULBitmap](`ultralight_sys::ULBitmap`) type.
///
/// [ULBitmap](`ultralight_sys::ULBitmap`) is a reference-counted bitmap class. It can be created from a [ULSurface](`crate::types::ULSurface`) or from a raw pointer.
/// Sometimes, it is necessary to create a [ULBitmap](`ultralight_sys::ULBitmap`) from a raw pointer, for example, when creating a [ULBitmap](`ultralight_sys::ULBitmap`) from a [ULSurface](`crate::types::ULSurface`).
/// In this case, the [ULBitmap](`ultralight_sys::ULBitmap`) is borrowed and does not need to be destroyed. However, when creating a [ULBitmap](`ultralight_sys::ULBitmap`) from a raw pointer, it is necessary to destroy it.
///
/// This wrapper handles the destruction of the [ULBitmap](`ultralight_sys::ULBitmap`) when it is no longer needed and uses its internal fields to describe these relationships.
///
/// # Example
/// ```rust
/// use ultralight::{Bitmap, BitmapFormat};
///
/// let bitmap = Bitmap::create_borrowed(100, 100, BitmapFormat::Bgra8UnormSrgb).unwrap();
/// assert_eq!(bitmap.format().unwrap(), BitmapFormat::Bgra8UnormSrgb);
/// ```
#[derive(Debug)]
pub enum Bitmap {
    #[doc(hidden)]
    Borrowed {
        internal: ultralight_sys::ULBitmap,
        need_to_destroy: bool,
    },
    #[doc(hidden)]
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

unsafe impl Send for Bitmap {}
unsafe impl Sync for Bitmap {}

impl LockablePixels for Bitmap {
    unsafe fn raw_lock_pixels(&mut self) -> *mut u8 {
        match self {
            Self::Borrowed { internal, .. } => ulBitmapRawPixels(*internal) as _,
            Self::Owned { .. } => panic!("Cannot lock pixels for owned bitmap"),
        }
    }

    unsafe fn raw_unlock_pixels(&mut self) {
        match self {
            Self::Borrowed { internal, .. } => ulBitmapUnlockPixels(*internal),
            Self::Owned { .. } => {}
        }
    }
}

/// Bitmap implementation.
impl Bitmap {
    /// Creates a new [Bitmap](`crate::types::Bitmap`) from a raw pointer.
    ///
    /// # Safety
    /// This function is unsafe because it creates a [Bitmap](`crate::types::Bitmap`) from a raw pointer. To satisfy the safety contract, the following conditions must be met:
    /// - The pointer must be non-null.
    /// - The pointer must be a valid [ULBitmap](`ultralight_sys::ULBitmap`).
    /// - The pointer must be a valid [ULBitmap](`ultralight_sys::ULBitmap`) that is not already borrowed.
    /// - The [ULBitmap](`ultralight_sys::ULBitmap`) must be destroyed after the [Bitmap](`crate::types::Bitmap`) is no longer needed, if required.
    /// - The [ULBitmap](`ultralight_sys::ULBitmap`) must not be destroyed while the [Bitmap](`crate::types::Bitmap`) is still in use.
    ///
    /// # Example
    /// ```rust
    /// use ultralight::{Bitmap, BitmapFormat};
    ///
    /// let raw_bitmap = unsafe { ultralight_sys::ulCreateBitmap(100, 100, BitmapFormat::Bgra8UnormSrgb as _) };
    /// let bitmap = unsafe { Bitmap::from_raw(raw_bitmap).unwrap() };
    /// assert_eq!(bitmap.format().unwrap(), BitmapFormat::Bgra8UnormSrgb);
    /// ```
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

    /// If the [Bitmap](`crate::types::Bitmap`) is owned, returns a reference to its pixels.
    pub fn pixels(&self) -> ULResult<&Vec<u8>> {
        match self {
            Self::Borrowed { internal, .. } => {
                Err(ULError::BitmapUnsupportedOperationForBorrowedBitmap)
            }
            Self::Owned { pixels, .. } => pixels.as_ref().ok_or(ULError::BitmapNullReference),
        }
    }

    /// Constructs an empty, borrowed [Bitmap](`crate::types::Bitmap`). The [Bitmap](`crate::types::Bitmap`) will be destroyed after it is no longer needed.
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

    /// Constructs a borrowed [Bitmap](`crate::types::Bitmap`) with the specified dimensions and format. The [Bitmap](`crate::types::Bitmap`) will be destroyed after it is no longer needed.
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

    /// Constructs a borrowed [Bitmap](`crate::types::Bitmap`) with the specified dimensions, format, and row bytes. The [Bitmap](`crate::types::Bitmap`) will be destroyed after it is no longer needed.
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

    /// Creates a cloned [Bitmap](`crate::types::Bitmap`) from the specified [Bitmap](`crate::types::Bitmap`). The [Bitmap](`crate::types::Bitmap`) will be destroyed after it is no longer needed.
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

    /// Gets the [Bitmap's](`crate::types::Bitmap`) width.
    pub fn width(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetWidth(*internal) },
            Self::Owned { width, .. } => *width,
        }
    }

    /// Gets the [Bitmap's](`crate::types::Bitmap`) height.
    pub fn height(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetHeight(*internal) },
            Self::Owned { height, .. } => *height,
        }
    }

    /// Gets the [Bitmap's](`crate::types::Bitmap`) format.
    pub fn format(&self) -> ULResult<BitmapFormat> {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetFormat(*internal) }.try_into(),
            Self::Owned { format, .. } => Ok(*format),
        }
    }

    /// Gets the [Bitmap's](`crate::types::Bitmap`) bits per pixel.
    pub fn bpp(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetBpp(*internal) },
            Self::Owned { bpp, .. } => *bpp,
        }
    }

    /// Gets the [Bitmap's](`crate::types::Bitmap`) row byte length.
    pub fn row_bytes(&self) -> u32 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetRowBytes(*internal) },
            Self::Owned { row_bytes, .. } => *row_bytes,
        }
    }

    /// Gets the [Bitmap's](`crate::types::Bitmap`) byte length.
    pub fn bytes_size(&self) -> u64 {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapGetSize(*internal) }
                .try_into()
                .expect("Failed to convert size to u64"),
            Self::Owned { bytes_size, .. } => *bytes_size,
        }
    }

    /// Gets whether the [Bitmap](`crate::types::Bitmap`) is empty.
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapIsEmpty(*internal) },
            Self::Owned { is_empty, .. } => *is_empty,
        }
    }

    /// Locks the [Bitmap's](`crate::types::Bitmap`) pixels. Returns a [PixelsGuard](`crate::types::PixelsGuard`) that unlocks the pixels when dropped. If the [Bitmap](`crate::types::Bitmap`) is owned, returns `None`.
    pub fn lock_pixels(&mut self) -> Option<PixelsGuard<Bitmap>> {
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

    pub(crate) fn raw_unlock_pixels(&mut self) {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapUnlockPixels(*internal) },
            Self::Owned { .. } => {}
        }
    }

    pub(crate) fn erase(&self) {
        match self {
            Self::Borrowed { internal, .. } => unsafe { ulBitmapErase(*internal) },
            Self::Owned { .. } => {}
        }
    }

    /// Writes the [Bitmap](`crate::types::Bitmap`) to a PNG file at the specified path.
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

    /// Swaps the red and blue channels of the [Bitmap](`crate::types::Bitmap`). Only works for [Bitmaps](`crate::types::Bitmap`) with the [Bgra8UnormSrgb](`crate::types::BitmapFormat::Bgra8UnormSrgb`) format.
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

    /// Converts the [Bitmap](`crate::types::Bitmap`) to an owned [Bitmap](`crate::types::Bitmap`).
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

    /// Converts the [Bitmap](`crate::types::Bitmap`) to a borrowed [Bitmap](`crate::types::Bitmap`).
    ///
    /// Currently unimplemented.
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
