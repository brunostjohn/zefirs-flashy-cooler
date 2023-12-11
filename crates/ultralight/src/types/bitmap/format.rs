use ultralight_sys::{
    ULBitmapFormat, ULBitmapFormat_kBitmapFormat_A8_UNORM,
    ULBitmapFormat_kBitmapFormat_BGRA8_UNORM_SRGB,
};

use crate::error::ULError;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BitmapFormat {
    A8Unorm = ULBitmapFormat_kBitmapFormat_A8_UNORM as isize,
    Bgra8UnormSrgb = ULBitmapFormat_kBitmapFormat_BGRA8_UNORM_SRGB as isize,
}

impl TryFrom<ULBitmapFormat> for BitmapFormat {
    type Error = ULError;

    #[allow(non_upper_case_globals)]
    fn try_from(value: ULBitmapFormat) -> Result<Self, Self::Error> {
        match value {
            ULBitmapFormat_kBitmapFormat_A8_UNORM => Ok(Self::A8Unorm),
            ULBitmapFormat_kBitmapFormat_BGRA8_UNORM_SRGB => Ok(Self::Bgra8UnormSrgb),
            _ => Err(ULError::BitmapUnsupportedFormat),
        }
    }
}

impl BitmapFormat {
    pub(crate) fn bytes_per_pixel(&self) -> usize {
        match self {
            Self::A8Unorm => 1,
            Self::Bgra8UnormSrgb => 4,
        }
    }
}
