pub enum ULFaceWinding {
    CounterClockwise,
    Clockwise,
}

impl From<ULFaceWinding> for ultralight_sys::ULFaceWinding {
    fn from(face_winding: ULFaceWinding) -> Self {
        match face_winding {
            ULFaceWinding::CounterClockwise => {
                ultralight_sys::ULFaceWinding_kFaceWinding_CounterClockwise
            }
            ULFaceWinding::Clockwise => ultralight_sys::ULFaceWinding_kFaceWinding_Clockwise,
        }
    }
}
