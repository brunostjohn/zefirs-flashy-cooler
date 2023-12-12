use crate::{
    bitmap::{Bitmap, LockablePixels, PixelsGuard},
    error::ULError,
    ULResult,
};

pub struct ULSurface(ultralight_sys::ULSurface);

unsafe impl Send for ULSurface {}
unsafe impl Sync for ULSurface {}

pub struct ULSurfaceBounds {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<ULSurfaceBounds> for ultralight_sys::ULIntRect {
    fn from(bounds: ULSurfaceBounds) -> Self {
        Self {
            left: bounds.left,
            top: bounds.top,
            right: bounds.right,
            bottom: bounds.bottom,
        }
    }
}

impl From<ultralight_sys::ULIntRect> for ULSurfaceBounds {
    fn from(bounds: ultralight_sys::ULIntRect) -> Self {
        Self {
            left: bounds.left,
            top: bounds.top,
            right: bounds.right,
            bottom: bounds.bottom,
        }
    }
}

impl LockablePixels for ULSurface {
    unsafe fn raw_lock_pixels(&mut self) -> *mut u8 {
        ultralight_sys::ulSurfaceLockPixels(self.0) as _
    }

    unsafe fn raw_unlock_pixels(&mut self) {
        ultralight_sys::ulSurfaceUnlockPixels(self.0)
    }
}

impl ULSurface {
    pub unsafe fn from_raw(surface: ultralight_sys::ULSurface) -> Self {
        assert!(!surface.is_null(), "ULSurface is null");
        Self(surface)
    }

    pub fn get_bitmap(&mut self) -> ULResult<Bitmap> {
        unsafe {
            let bitmap = ultralight_sys::ulBitmapSurfaceGetBitmap(self.0);
            Bitmap::from_raw(bitmap).ok_or(ULError::BitmapNullReference)
        }
    }

    pub fn get_width(&self) -> u32 {
        unsafe { ultralight_sys::ulSurfaceGetWidth(self.0) }
    }

    pub fn get_height(&self) -> u32 {
        unsafe { ultralight_sys::ulSurfaceGetHeight(self.0) }
    }

    pub fn get_row_bytes(&self) -> u32 {
        unsafe { ultralight_sys::ulSurfaceGetRowBytes(self.0) }
    }

    pub fn get_size(&self) -> usize {
        unsafe { ultralight_sys::ulSurfaceGetSize(self.0) }
    }

    pub fn lock_pixels(&mut self) -> ULResult<PixelsGuard<ULSurface>> {
        unsafe {
            let pixels = self.raw_lock_pixels();
            let pixels = std::slice::from_raw_parts_mut(pixels, self.get_size());
            Ok(PixelsGuard::new(self, pixels))
        }
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe { ultralight_sys::ulSurfaceResize(self.0, width, height) }
    }

    pub fn set_dirty_bounds(&mut self, bounds: ULSurfaceBounds) {
        unsafe { ultralight_sys::ulSurfaceSetDirtyBounds(self.0, bounds.into()) }
    }

    pub fn get_dirty_bounds(&self) -> ULSurfaceBounds {
        unsafe { ultralight_sys::ulSurfaceGetDirtyBounds(self.0).into() }
    }

    pub fn clear_dirty_bounds(&mut self) {
        unsafe { ultralight_sys::ulSurfaceClearDirtyBounds(self.0) }
    }
}
