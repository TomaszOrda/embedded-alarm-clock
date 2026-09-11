#![no_std]
#![no_main]

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32l0xx_hal::{pac, prelude::*, rcc::Config};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.freeze(Config::hsi16());

    let gpioa = dp.GPIOB.split(&mut rcc);
    let mut led = gpioa.pb3.into_push_pull_output();

    loop {
        led.set_low().unwrap();
        delay(1_000_000);
        led.set_high().unwrap();
        delay(1_000_000);
        
    }
}
