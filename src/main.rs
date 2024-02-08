#![no_main]
#![no_std]

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};
use panic_halt as _;

#[entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();

    let gpioa = p.GPIOA.split();
    let mut led = gpioa.pa5.into_push_pull_output();

    loop {
        for _ in 0..10000 {
            led.set_high();
        }

        for _ in 0..10000 {
            led.set_low();
        }
    }
}
