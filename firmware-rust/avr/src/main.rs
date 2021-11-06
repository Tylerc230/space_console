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
    let mut screen = Screen {led_strip1: ws};
    loop {
        runner.update(&mut buffer);
        screen.write_buffer(&buffer);
        arduino_hal::delay_ms(1000 as u16);
    }
}

struct Screen<PIN1> where PIN1: OutputPin {
    led_strip1: Ws2812<Timer, PIN1>
}

impl<PIN1: OutputPin> Screen<PIN1> {
    fn write_buffer(&mut self, buffer: &fr::PixelBuffer) {
        self.led_strip1.write(buffer.pixels[0].iter().cloned()).unwrap();
    }
}


