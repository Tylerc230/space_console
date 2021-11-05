#![no_std]
#![no_main]

use panic_halt as _;
use firmware_rust as fr;
use ws2812_timer_delay::Ws2812;
mod hal_timer;
use hal_timer::{
    Timer, MicroSeconds
};
use fr::smart_leds::{SmartLedsWrite, RGB8};
use embedded_hal::digital::v2::OutputPin;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut outpin = pins.d3.into_output();
    let timer = Timer{micro_seconds: MicroSeconds(1)};
    let mut ws = Ws2812::new(timer, outpin);

    let simple = fr::programs::simple_program::SimpleProgram {};
    let runner = fr::ProgramRunner { current_program: &simple};
    let mut buffer = fr::PixelBuffer::new();
    let mut screen = Screen {pixel_row1: ws};
    loop {
        runner.update(&mut buffer);
        //write_buffer(&mut ws, &buffer.pixels[0]);
        screen.write_buffer(&buffer.pixels[0]);
        arduino_hal::delay_ms(1000 as u16);
    }
}

fn write_buffer<PIN>(ws: &mut Ws2812<Timer, PIN>, buffer: &[RGB8]) where PIN: OutputPin {
    ws.write(buffer.iter().cloned()).unwrap();
}

struct Screen<PIN1> where PIN1: OutputPin {
    pixel_row1: Ws2812<Timer, PIN1>
}

impl<PIN1: OutputPin> Screen<PIN1> {
    fn write_buffer(&mut self, buffer: &[RGB8]) {
        self.pixel_row1.write(buffer.iter().cloned()).unwrap();
    }
}


