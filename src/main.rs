//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

use panic_halt as _;

use stm32f4xx_hal::{
    prelude::*,
    pwm,
    stm32,
};
//use cortex_m_semihosting::hprintln;
use cortex_m_rt::entry;


#[entry]
fn main() -> ! {
    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = stm32::Peripherals::take().unwrap();

    // Take ownership over the raw rcc device and convert it into the corresponding
    // HAL structs
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in 'clocks'
    let clocks = rcc.cfgr.freeze();

    // Acquire the GPIOA peripheral
    let gpioa = dp.GPIOA.split();

    //from https://github.com/thalesfragoso/stm32f4xx-hal/blob/pwm-impl/examples/pwm.rs
    //TODO: keep checking for the merge of this into 
    let channels = (
        gpioa.pa8.into_alternate_af1(),
        gpioa.pa9.into_alternate_af1(),
    );

    //all channels share the same base frequency, but the duty cycle can be changed

    //see: https://docs.rs/embedded-hal/0.2.1/embedded_hal/trait.Pwm.html

    let pwm = pwm::tim1(dp.TIM1, channels, clocks, 1.khz());

    //max_duty should be the same for all channela ???
    //let max_duty = pwm.get_max_duty();  //not allowed. Intellisense seems to indicate only 2 channels on stm32f411 (I believe max 4 channels on stm32)

    let (mut ch1, mut ch2) = pwm;

    ch1.set_duty(ch1.get_max_duty() / 2);
    ch1.enable();

    ch2.set_duty(ch2.get_max_duty() / 8);
    ch2.enable();    


    loop {
        cortex_m::asm::nop();
    }
}