use thiserror::Error;

#[derive(Error, Debug)]
pub enum ULError {
    #[error("Failed to start remote inspector server")]
    FailedToStartInspectorServer,
    #[error("Failed to load webpage")]
    FailedToLoadWebpage,
    #[error("Creation of bitmap failed because Ultralight returned a null pointer")]
    BitmapNullReference,
    #[error(
        "Creation of bitmap failed because it required {required} bytes, but got {got} bytes only"
    )]
    BitmapPixelBufferSizeMismatch { got: usize, required: usize },
    #[error("Tried to swap red and blue channels on an unsupported format")]
    BitmapUnsupportedOperationForPixelFormat,
    #[error("Could not write bitmap to PNG successfully")]
    BitmapFailedPngWrite,
    #[error("Could not create bitmap because its empty")]
    BitmapEmptyBitmap,
    #[error("Could not create bitmap because its format is not supported")]
    BitmapUnsupportedFormat,
    #[error("Could not save bitmap to file because of invalid path")]
    BitmapPNGInvalidPath,
    #[error("This is an unsupported operation for an owned bitmap")]
    BitmapUnsupportedOperationForOwnedBitmap,
    #[error("Failed to convert vertex buffer format due to an unsupported one being provided")]
    VertexBufferUnsupportedFormat,
    #[error("Failed to convert vertex buffer due to a null pointer being provided")]
    VertexBufferNullReference,
    #[error("Failed to convert shader type due to an unsupported one being provided")]
    ShaderUnsupportedType,
    #[error("Failed to convert GPU command due to an unsupported one being provided")]
    GPUCommandUnsupportedType,
    #[error("Failed to start GPU driver due to a lack of a supported GPU adapter")]
    GPUDriverNoCompatibleAdapter,
    #[error("Failed to start GPU driver due to a lack of a supported GPU device")]
    GPUDriverNoCompatibleDevice,
    #[error("Failed to perform operation as it is unsupported for a borrowed bitmap")]
    BitmapUnsupportedOperationForBorrowedBitmap,
    #[error("Failed to get geometry from map")]
    GPUFailedToGetGeometry,
    #[error("Failed to get render buffer from map")]
    GPUFailedToGetRenderBuffer,
    #[error("JS value type not known")]
    JSValueTypeNotKnown,
    #[error("JS value type invalid for this cast")]
    JSInvalidCast,
}
