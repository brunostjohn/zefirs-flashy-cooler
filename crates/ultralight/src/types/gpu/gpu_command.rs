use ultralight_sys::ULCommand;

use crate::error::ULError;

use super::gpu_state::GPUState;

#[derive(Debug)]
pub enum GPUCommand {
    ClearRenderBuffer {
        render_buffer_id: u32,
    },
    DrawGeometry {
        gpu_state: Box<GPUState>,
        geometry_id: u32,
        indices_offset: u32,
        indices_count: u32,
    },
}

impl TryFrom<ULCommand> for GPUCommand {
    type Error = ULError;

    #[inline]
    #[allow(non_upper_case_globals)]
    #[allow(non_snake_case)]
    fn try_from(gc: ULCommand) -> Result<Self, Self::Error> {
        match gc.command_type as u32 {
            ULCommandType_kCommandType_DrawGeometry => Ok(GPUCommand::DrawGeometry {
                gpu_state: Box::new(GPUState::try_from(gc.gpu_state)?),
                geometry_id: gc.geometry_id,
                indices_count: gc.indices_count,
                indices_offset: gc.indices_offset,
            }),
            ULCommandType_kCommandType_ClearRenderBuffer => Ok(GPUCommand::ClearRenderBuffer {
                render_buffer_id: gc.gpu_state.render_buffer_id,
            }),
            _ => Err(ULError::GPUCommandUnsupportedType),
        }
    }
}
