// make `std` available when testing
#![cfg_attr(not(test), no_std)]
mod program_runner;
mod program;
mod pixel_buffer;
pub use program_runner::ProgramRunner;
pub use program::Program;
pub use pixel_buffer::PixelBuffer;
