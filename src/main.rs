#![no_std]
#![no_main]

use arduino_uno::prelude::*;
use panic_halt as _;

#[arduino_uno::entry]
fn main() -> ! {
    let dp = arduino_uno::Peripherals::take().unwrap();
    let mut pins = arduino_uno::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD);
    // set up the serial port
    let mut serial = arduino_uno::Serial::new(
        dp.USART0,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate(),
    );
    // output startup message
    ufmt::uwriteln!(&mut serial, "Hall Effect Sensor 0.2.0\n").void_unwrap();
    // set up the hall effect sensor pin
    let sensor = pins.d2.into_pull_up_input(&mut pins.ddr);
    let mut pin_low = false;

    loop {
        if sensor.is_low().unwrap() && !pin_low {
            ufmt::uwriteln!(&mut serial, "Detected").void_unwrap();
            pin_low = true;
        } else if sensor.is_high().unwrap() && pin_low {
            pin_low = false;
        }
    }
}


