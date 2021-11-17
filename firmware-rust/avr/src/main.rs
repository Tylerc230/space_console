#![no_std]
#![no_main]

use panic_halt as _;
use firmware_rust as fr;
mod serial;
mod led_driver;
mod hal_timer;

use void::ResultVoidExt;
use rotary_encoder_hal::{Direction, Rotary};
use led_driver::{
    LEDStrip,
    Screen
};

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let serial = arduino_hal::default_serial!(dp, pins, 57600);
    serial::init(serial);
    serial_println!("Hello from Arduino!\r").void_unwrap();

    let mut simple = fr::programs::simple_program::SimpleProgram::new();
    let mut runner = fr::ProgramRunner::new(&mut simple);

    let mut led_strip0 = LEDStrip::new(pins.d4.into_output());
    let mut led_strip1 = LEDStrip::new(pins.d2.into_output());//not working
    let mut led_strip2 = LEDStrip::new(pins.d3.into_output());
    let mut led_strip3 = LEDStrip::new(pins.d5.into_output());
    let mut led_strip4 = LEDStrip::new(pins.d6.into_output());
    let mut screen = Screen {led_strips: [
        &mut led_strip0,
        &mut led_strip1,
        &mut led_strip2,
        &mut led_strip3,
        &mut led_strip4,
    ] };

    //knob A (prog select) arduino pin 10, 11
    //knob B broken (mod select) adruino pin 12, 17
    //outer
    let pin_a = pins.d11.into_pull_up_input();
    let pin_b = pins.d10.into_pull_up_input();
    //let switch = pins.a1.into_pull_up_input();
    //inner
    //let pin_a = pins.d12.into_pull_up_input();
    //let pin_b = pins.a3.into_pull_up_input();
    //let switch = pins.a2.into_pull_up_input();
    let mut enc = Rotary::new(pin_a, pin_b);
    loop {
        match enc.update().unwrap() {
            Direction::Clockwise => {
                serial_println!("CW\r").void_unwrap();
            }
            Direction::CounterClockwise => {
                serial_println!("CCW\r").void_unwrap();
            }
            _ => {}
        }
        runner.update();
        screen.write_buffer(&runner.pixel_buffer);
        //arduino_hal::delay_ms(1000 as u16);
    }
}

struct InputDriver {

}

