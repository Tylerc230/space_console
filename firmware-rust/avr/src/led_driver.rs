use firmware_rust as fr;
use ws2812_timer_delay::Ws2812;
use embedded_hal::digital::v2::OutputPin;
use super::hal_timer::{
    Timer, MicroSeconds
};
use fr::smart_leds::{SmartLedsWrite, RGB8};
pub struct Screen<'a>  {
    pub led_strips: [&'a mut dyn RGB8Writable; fr::PixelBuffer::NUM_LED_STRIPS]
}

impl<'a> Screen<'a> {
    pub fn write_buffer(&mut self, buffer: &fr::PixelBuffer) {

        self.led_strips[0].write(&buffer.pixels[0]);
        self.led_strips[1].write(&buffer.pixels[1]);
        self.led_strips[2].write(&buffer.pixels[2]);
        self.led_strips[3].write(&buffer.pixels[3]);
        self.led_strips[4].write(&buffer.pixels[4]);
        //crashes in loop
        //for i in 0..5 {
            //self.led_strips[i].write(&buffer.pixels[i]);
        //}
        //self.led_strips[0].write(&buffer.pixels[0]);
        //buffer
            //.pixels
            //.iter()
            //.zip(self.led_strips.iter_mut())
            //.for_each(|tup| {
                //tup.1.write(tup.0);
            //});
        //
    }
}

pub trait RGB8Writable {
    fn write(&mut self, led_colors: &[RGB8; fr::PixelBuffer::LEDS_PER_STRIP]);
}

pub struct LEDStrip<PIN> where PIN: OutputPin {
    led_strip: Ws2812<Timer, PIN>
}

impl<PIN: OutputPin> LEDStrip<PIN> {
    pub fn new(pin: PIN) -> Self {
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
