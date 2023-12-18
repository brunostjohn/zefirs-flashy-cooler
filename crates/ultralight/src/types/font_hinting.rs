pub enum ULFontHinting {
    Smooth,
    Normal,
    Monochrome,
}

impl From<ULFontHinting> for ultralight_sys::ULFontHinting {
    fn from(font_hinting: ULFontHinting) -> Self {
        match font_hinting {
            ULFontHinting::Smooth => ultralight_sys::ULFontHinting_kFontHinting_Smooth,
            ULFontHinting::Normal => ultralight_sys::ULFontHinting_kFontHinting_Normal,
            ULFontHinting::Monochrome => ultralight_sys::ULFontHinting_kFontHinting_Monochrome,
        }
    }
}
