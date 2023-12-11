use std::slice;

use ultralight_sys::{
    ULVertexBuffer, ULVertexBufferFormat, ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f,
    ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f,
};

use crate::error::ULError;

#[allow(non_camel_case_types)]
pub enum VertexBufferFormat {
    Format_2f_4ub_2f = ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f as isize,
    Format_2f_4ub_2f_2f_28f = ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f as isize,
}

impl TryFrom<ULVertexBufferFormat> for VertexBufferFormat {
    type Error = ULError;

    #[allow(non_upper_case_globals)]
    fn try_from(vbf: ULVertexBufferFormat) -> Result<Self, Self::Error> {
        match vbf {
            ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f)
            }
            ULVertexBufferFormat_kVertexBufferFormat_2f_4ub_2f_2f_28f => {
                Ok(VertexBufferFormat::Format_2f_4ub_2f_2f_28f)
            }
            _ => Err(ULError::VertexBufferUnsupportedFormat),
        }
    }
}

pub(crate) struct VertexBuffer {
    pub(crate) format: VertexBufferFormat,
    pub(crate) buffer: Vec<u8>,
}

impl TryFrom<ULVertexBuffer> for VertexBuffer {
    type Error = ULError;

    fn try_from(vb: ULVertexBuffer) -> Result<Self, Self::Error> {
        if vb.data.is_null() {
            return Err(ULError::VertexBufferNullReference);
        }
        let format = VertexBufferFormat::try_from(vb.format)?;
        let buffer = unsafe { slice::from_raw_parts(vb.data, vb.size as usize) };
        Ok(VertexBuffer {
            format,
            buffer: buffer.to_vec(),
        })
    }
}
