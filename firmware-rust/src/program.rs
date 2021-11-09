use crate::pixel_buffer::PixelBuffer;
pub trait Program {
    fn init(&self);
    fn update(&mut self, buffer: &mut PixelBuffer);
}

