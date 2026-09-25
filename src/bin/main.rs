#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use embassy_time::{Timer, Duration};
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Output, OutputConfig};
use log::info;
use embassy_executor::Spawner;
use embedded_alarm_clock::buzzer::Buzzer;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(_spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o esp32c3-mini-1 -o log -o esp-backtrace -o wokwi -o vscode -o stable-x86_64-pc-windows-msvc

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);
    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO2
    // - GPIO8
    // - GPIO9
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO12;
    let _ = peripherals.GPIO13;
    let _ = peripherals.GPIO14;
    let _ = peripherals.GPIO15;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO17;

    let mut led = Output::new(peripherals.GPIO2,esp_hal::gpio::Level::Low, OutputConfig::default());
    let button: Input<'_> = Input::new(peripherals.GPIO4, InputConfig::default().with_pull(esp_hal::gpio::Pull::None));
    
    let mut buzzer: Buzzer = Buzzer::new(Output::new(peripherals.GPIO3,esp_hal::gpio::Level::Low, OutputConfig::default()));
    
    let mut loop_index = 0;
    loop {
        Timer::after(Duration::from_millis(1000)).await;
        info!("High");
        led.set_high();
        if loop_index % 10 == 0{
            buzzer.buzz(10, 0.5, &button).await;
        }
        Timer::after(Duration::from_millis(1000)).await;
        info!("Low");
        led.set_low();
        loop_index = loop_index + 1;
    }

}

