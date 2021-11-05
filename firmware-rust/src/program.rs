use crate::pixel_buffer::PixelBuffer;
pub trait Program {
    fn init(&self);
    fn update(&self, buffer: &mut PixelBuffer);
}

