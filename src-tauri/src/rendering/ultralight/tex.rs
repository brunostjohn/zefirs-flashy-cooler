use glium::{
    framebuffer::ToColorAttachment,
    pixel_buffer::PixelBuffer,
    texture::SrgbTexture2d,
    uniforms::{AsUniformValue, Sampler, UniformValue},
    Texture2d,
};

pub enum EitherSampler<'t> {
    Regular2d(Sampler<'t, Texture2d>),
    Srgb2d(Sampler<'t, SrgbTexture2d>),
}

impl<'a> AsUniformValue for EitherSampler<'a> {
    #[inline]
    fn as_uniform_value(&self) -> UniformValue {
        match self {
            EitherSampler::Regular2d(t) => t.as_uniform_value(),
            EitherSampler::Srgb2d(t) => t.as_uniform_value(),
        }
    }
}

pub enum EitherTexture {
    Regular2d(Texture2d),
    Srgb2d(SrgbTexture2d),
}

impl EitherTexture {
    #[inline]
    pub fn width(&self) -> u32 {
        match self {
            EitherTexture::Regular2d(t) => t.width(),
            EitherTexture::Srgb2d(t) => t.width(),
        }
    }

    #[inline]
    pub fn height(&self) -> u32 {
        match self {
            EitherTexture::Regular2d(t) => t.height(),
            EitherTexture::Srgb2d(t) => t.height(),
        }
    }

    #[inline]
    pub fn sampled(&'_ self) -> EitherSampler<'_> {
        match self {
            EitherTexture::Regular2d(t) => EitherSampler::Regular2d(t.sampled()),
            EitherTexture::Srgb2d(t) => EitherSampler::Srgb2d(t.sampled()),
        }
    }

    #[inline]
    pub fn data(&'_ self) -> PixelBuffer<(u8, u8, u8, u8)> {
        match self {
            EitherTexture::Regular2d(t) => t.read_to_pixel_buffer(),
            EitherTexture::Srgb2d(t) => t.read_to_pixel_buffer(),
        }
    }
}

impl<'t> ToColorAttachment<'t> for &'t EitherTexture {
    fn to_color_attachment(self) -> glium::framebuffer::ColorAttachment<'t> {
        match self {
            EitherTexture::Regular2d(t) => t.to_color_attachment(),
            EitherTexture::Srgb2d(t) => t.to_color_attachment(),
        }
    }
}
