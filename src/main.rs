#![no_std]
#![no_main]

use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_hal::gpio::{Output, OutputConfig};
use core::panic::PanicInfo;

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let peripherals = esp_hal::init(esp_hal::Config::default());
    let mut led = Output::new(peripherals.GPIO0,esp_hal::gpio::Level::Low, OutputConfig::default());
    loop{
        Timer::after(Duration::from_secs(2)).await;
        led.set_high();
        Timer::after(Duration::from_secs(2)).await;
        led.set_low();

    }
    // let dp = pac::Peripherals::take().unwrap();

    // let mut rcc = dp.RCC.freeze(Config::hsi16());

    // let gpiob = dp.GPIOB.split(&mut rcc);
    // let mut led = gpiob.pb3.into_push_pull_output();

    // loop {
    //     led.set_low().unwrap();
    //     delay(1_000_000);
    //     led.set_high().unwrap();
    //     delay(1_000_000);
        
    // }
}
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // let mut host_stderr = HStderr::new();

    // logs "panicked at '$reason', src/main.rs:27:4" to the host stderr
    // writeln!("something", "{}", info).ok();

    loop {}
}