use std::ops::{Deref, DerefMut};

/// A trait for types that can be locked for pixel access.
pub trait LockablePixels {
    /// Locks the pixels for access.
    ///
    /// # Safety
    /// - Must return a non-null pointer.
    /// - Must be balanced with a call to `raw_unlock_pixels`.
    /// - The returned pointer must be valid for the lifetime of the lock.
    /// - The returned pointer must be aligned to the value returned by `raw_bytes_per_row`.
    /// - The returned pointer must point to a buffer of at least `raw_size` bytes.
    unsafe fn raw_lock_pixels(&mut self) -> *mut u8;
    /// Unlocks the pixels.
    ///
    /// # Safety
    /// - Must be balanced with a call to `raw_lock_pixels`.
    /// - The pointer returned by `raw_lock_pixels` must be valid for the lifetime of the lock.
    /// - The pointer returned by `raw_lock_pixels` must be aligned to the value returned by `raw_bytes_per_row`.
    unsafe fn raw_unlock_pixels(&mut self);
}

/// A guard that locks pixels for access.
pub struct PixelsGuard<'a, T: LockablePixels> {
    lock: &'a mut T,
    pixels: &'a mut [u8],
}

impl<'a, T: LockablePixels> PixelsGuard<'a, T> {
    pub(crate) unsafe fn new(lock: &'a mut T, pixels: &'a mut [u8]) -> PixelsGuard<'a, T> {
        PixelsGuard { lock, pixels }
    }
}

impl<T: LockablePixels> Deref for PixelsGuard<'_, T> {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.pixels
    }
}

impl<T: LockablePixels> DerefMut for PixelsGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut [u8] {
        self.pixels
    }
}

impl<T: LockablePixels> Drop for PixelsGuard<'_, T> {
    fn drop(&mut self) {
        unsafe {
            self.lock.raw_unlock_pixels();
        }
    }
}
