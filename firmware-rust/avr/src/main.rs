#![no_std]
#![no_main]

use panic_halt as _;
use firmware_rust as fr;
mod serial;
mod led_driver;
mod hal_timer;
mod input_driver;

use void::ResultVoidExt;
use led_driver::{
    LEDStrip,
    Screen
};
use rotary_encoder_hal::Rotary;
use firmware_rust::ufmt;
use input_driver::InputDriver;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins  = arduino_hal::pins!(dp);

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
    //prog select (outer) switch (a1)
    let left_rotary = Rotary::new(pins.d11.into_pull_up_input(), pins.d10.into_pull_up_input());
    //let switch = pins.a1.into_pull_up_input();

    //mod select (inner) switch (a2)
    let right_rotary = Rotary::new(pins.d12.into_pull_up_input(), pins.a3.into_pull_up_input());
    //let switch = pins.a2.into_pull_up_input();
    let mut input = InputDriver::new(left_rotary, right_rotary);
    loop {
        let input_values = input.update();
        if input_values.left_rotary_direction != fr::input::KnobDirection::None || input_values.right_rotary_direction != fr::input::KnobDirection::None {
            serial_println!("Input {:#?}\r", input_values).void_unwrap();
        }
        runner.update();
        screen.write_buffer(&runner.pixel_buffer);
        //arduino_hal::delay_ms(1000 as u16);
    }
}

