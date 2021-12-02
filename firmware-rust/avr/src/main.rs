#![no_std]
#![feature(abi_avr_interrupt)]
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

static mut INPUT_DRIVER: Option<InputDriver> = None;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins  = arduino_hal::pins!(dp);

    dp.EXINT.pcicr.write(|w| {
        w.pcie().bits(0b00000011u8)//Enable port B and C
    });

    dp.EXINT.pcmsk0.write(|w|{//port B
        w.pcint().bits(0b00011100u8)//PB2, PB3, PB4 (ie d10, d11, d12)
    });

    dp.EXINT.pcmsk1.write(|w| {
        w.pcint().bits(0b00001000u8)//PC3 (id a3)
    });

    unsafe {
        avr_device::interrupt::enable();
    }

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
    //right most direction is backwards
    //prog select (outer) switch (a1)
    let right_rotary = Rotary::new(pins.d11.into_pull_up_input(), pins.d10.into_pull_up_input());
    //let switch = pins.a1.into_pull_up_input();

    //mod select (inner) switch (a2)
    let left_rotary = Rotary::new(pins.d12.into_pull_up_input(), pins.a3.into_pull_up_input());
    //let switch = pins.a2.into_pull_up_input();
    unsafe {
        INPUT_DRIVER = Some(InputDriver::new(left_rotary, right_rotary));
    }
    loop {
        //unsafe {
            //let input_values = &INPUT_DRIVER.as_ref().unwrap().cur_input;
            //if input_values.left_rotary_direction != fr::input::KnobDirection::None || input_values.right_rotary_direction != fr::input::KnobDirection::None {
                //serial_println!("Input {:#?}\r", input_values).void_unwrap();
            //}
        //}
        runner.update();
        screen.write_buffer(&runner.pixel_buffer);
        arduino_hal::delay_ms(1000 as u16);
    }
}
#[avr_device::interrupt(atmega328p)]
fn PCINT0() { //or 1 or 2
    unsafe {
        if let Some(input) = INPUT_DRIVER.as_mut() {
            input.update();
        }
        let driver = INPUT_DRIVER.as_mut().unwrap();
        let input_values = driver.cur_input;
        serial_println!("pcint0 Input {:#?} pins {:#?}\r", input_values, driver.pin_values()).void_unwrap();
    }
}

#[avr_device::interrupt(atmega328p)]
fn PCINT1() {
    unsafe {
        if let Some(input) = INPUT_DRIVER.as_mut() {
            //input.update();//This crashes
        }
        let input_values = &INPUT_DRIVER.as_ref().unwrap().cur_input;
        serial_println!("pcint1 Input {:#?}\r", input_values).void_unwrap();
    }
}

