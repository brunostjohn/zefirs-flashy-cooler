use ultralight_sys::{
    ULShaderType, ULShaderType_kShaderType_Fill, ULShaderType_kShaderType_FillPath,
};

use crate::error::ULError;

#[derive(Debug)]
pub enum ShaderType {
    Fill = ULShaderType_kShaderType_Fill as isize,
    FillPath = ULShaderType_kShaderType_FillPath as isize,
}

impl TryFrom<ULShaderType> for ShaderType {
    type Error = ULError;

    #[allow(non_upper_case_globals)]
    #[inline(always)]
    fn try_from(st: ULShaderType) -> Result<Self, Self::Error> {
        match st {
            ULShaderType_kShaderType_Fill => Ok(ShaderType::Fill),
            ULShaderType_kShaderType_FillPath => Ok(ShaderType::FillPath),
            _ => Err(ULError::ShaderUnsupportedType),
        }
    }
}
