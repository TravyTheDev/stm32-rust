#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_halt as _;
use cortex_m_rt as rt;
use stm32f4xx_hal as hal;
use crate::hal::{pac, prelude::*};

#[rt::entry]
fn main() -> ! {
    let p = pac::Peripherals::take().unwrap();
    let mut rcc = p.RCC.constrain();
    let gpioa = p.GPIOA.split(&mut rcc);
    let mut led = gpioa.pa5.into_push_pull_output();
    
    loop {
        led.set_high();
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }
        led.set_low();
        for _ in 0..400_000 {
            cortex_m::asm::nop();
        }
    }
}