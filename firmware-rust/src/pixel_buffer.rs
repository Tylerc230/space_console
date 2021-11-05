use smart_leds::{
    RGB8,
    colors::*
};
const ROWS: usize = 2;
const COLUMNS: usize = 2;
#[derive(PartialEq, Eq, Debug)]
pub struct PixelBuffer {
    pub pixels: [[RGB8; COLUMNS]; ROWS]
}

impl PixelBuffer {
    pub fn new() -> PixelBuffer {
        PixelBuffer { pixels: [[BLACK; COLUMNS]; ROWS]}
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
