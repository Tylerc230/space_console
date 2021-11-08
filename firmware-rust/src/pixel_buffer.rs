use smart_leds::{
    RGB8,
    colors::*
};
#[derive(PartialEq, Eq, Debug)]
pub struct PixelBuffer {
    pub pixels: [[RGB8; Self::LEDS_PER_STRIP]; Self::NUM_LED_STRIPS]
}

impl PixelBuffer {
    pub const NUM_LED_STRIPS: usize = 1;
    pub const LEDS_PER_STRIP: usize = 2;
    pub fn new() -> PixelBuffer {
        Self::new_fill_color(BLACK)
    }

     pub fn new_fill_color(fill_color: RGB8) -> PixelBuffer {
        PixelBuffer { pixels: [[fill_color; Self::LEDS_PER_STRIP]; Self::NUM_LED_STRIPS]}
    }


     
    pub fn fill(&mut self, color: RGB8) {
        self.pixels
            .iter_mut()
            .flatten()
            .for_each(|pixel| {
                *pixel = color;
            });
    }
}
