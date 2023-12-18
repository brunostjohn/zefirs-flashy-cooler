use std::slice;
use ultralight_sys::ULIndexBuffer;

#[derive(Debug)]
pub struct IndexBuffer {
    pub buffer: Vec<u32>,
}

impl From<ULIndexBuffer> for IndexBuffer {
    #[inline(always)]
    fn from(vb: ULIndexBuffer) -> Self {
        debug_assert!(vb.size % 4 == 0);
        debug_assert!(!vb.data.is_null());
        let index_slice = unsafe { slice::from_raw_parts(vb.data as _, vb.size as usize / 4) };
        IndexBuffer {
            buffer: index_slice.to_vec(),
        }
    }
}
