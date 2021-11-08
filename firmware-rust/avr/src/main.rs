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

    let simple = fr::programs::simple_program::SimpleProgram {};
    let mut runner = fr::ProgramRunner::new(&simple);

    let mut led_strip1 = LEDStrip::new(pins.d3.into_output());
    let mut screen = Screen {led_strips: [&mut led_strip1] };
    loop {
        runner.update();
        screen.write_buffer(&runner.pixel_buffer);
        arduino_hal::delay_ms(1000 as u16);
    }
}

struct Screen<'a>  {
    led_strips: [&'a mut dyn RGB8Writable; fr::PixelBuffer::NUM_LED_STRIPS]
}

impl<'a> Screen<'a> {
    fn write_buffer(&mut self, buffer: &fr::PixelBuffer) {
        self.led_strips[0].write(&buffer.pixels[0]);
        //buffer
            //.pixels
            //.iter()
            //.zip(self.led_strips.iter_mut())
            //.for_each(|tup| {
                //tup.1.write(tup.0);
            //});
    }
}

trait RGB8Writable {
    fn write(&mut self, led_colors: &[RGB8; fr::PixelBuffer::LEDS_PER_STRIP]);
}

struct LEDStrip<PIN> where PIN: OutputPin {
    led_strip: Ws2812<Timer, PIN>
}

impl<PIN: OutputPin> LEDStrip<PIN> {
    fn new(pin: PIN) -> Self {
        let timer = Timer{micro_seconds: MicroSeconds(1)};
        let ws = Ws2812::new(timer, pin);
        LEDStrip{led_strip: ws}
    }

}

impl<PIN: OutputPin> RGB8Writable for LEDStrip<PIN> {
    fn write(&mut self, led_colors: &[RGB8; fr::PixelBuffer::LEDS_PER_STRIP]) {
        self.led_strip.write(led_colors.iter().cloned()).unwrap();
    }
}
