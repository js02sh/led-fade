
#![no_std]
#![no_main]

use arduino_hal::prelude::*;
use panic_halt as _;
use arduino_hal::simple_pwm::*;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut adc = arduino_hal::Adc::new(dp.ADC, Default::default());

    let a0 = pins.a0.into_analog_input(&mut adc);
    let timer0 = Timer0Pwm::new(dp.TC0, Prescaler::Prescale64);
    let mut pwm_led = pins.d5.into_output().into_pwm(&timer0);
    pwm_led.enable();
    
    loop {
        let x_temp = a0.analog_read(&mut adc)/4;
        let x: u8 = x_temp as u8;
        pwm_led.set_duty(x);

        ufmt::uwriteln!(&mut serial, "A: {} ", x_temp).void_unwrap();
        arduino_hal::delay_ms(100);
    }
}
