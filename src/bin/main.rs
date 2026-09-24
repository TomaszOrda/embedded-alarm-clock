#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]


use epd_waveshare::prelude::*;
use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config as I2CConfig};
use esp_hal::{i2c, main};
use esp_hal::time::{Duration, Instant, Rate};
use esp_hal::gpio::{Input, InputConfig, Level::{self}, Output, OutputConfig};
use esp_hal::spi::{Mode, master::{Config, Spi}};
use log::info;
use heapless::format;

use embedded_alarm_clock::epaper::EPaperDisplay;
use embedded_alarm_clock::rtc::RTC;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o esp32c3-mini-1 -o log -o esp-backtrace -o wokwi -o vscode -o stable-x86_64-pc-windows-msvc

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let i2c_sda = peripherals.GPIO6;
    let i2c_scl = peripherals.GPIO7;
    let spi_mosi = peripherals.GPIO10;
    let spi_sck = peripherals.GPIO8;
    let _u0rxd = peripherals.GPIO20;
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

    let i2c = i2c::master::I2c::new(peripherals.I2C0,I2CConfig::default()).unwrap()
                                                                                             .with_scl(i2c_scl)
                                                                                             .with_sda(i2c_sda);
    let mut rtc: RTC = RTC::new(i2c, peripherals.LPWR, #[cfg(not(debug_assertions))] peripherals.GPIO5.into_pull_up_input().into());

    let cs = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let busy = Input::new(peripherals.GPIO21, InputConfig::default());

    let spi = Spi::new(peripherals.SPI2, 
                       Config::default().with_mode(Mode::_0)
                                        .with_frequency(Rate::from_mhz(4)))
                                        .unwrap()
                                        .with_sck(spi_sck)
                                        .with_mosi(spi_mosi);
    let mut EPaper_display = EPaperDisplay::new(spi, cs, busy, dc, rst);
    EPaper_display.wait_till_idle().unwrap();
    info!("Display initialized");

    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(5000) {}
    info!("High");
    info!("Current time {}", rtc.get_time_hh_mm().unwrap_or(format!("??:??").unwrap()));
    EPaper_display.draw_circle(Color::Black);
    EPaper_display.flush();
    EPaper_display.wait_till_idle().unwrap();

    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(5000) {}
    info!("Low");
    EPaper_display.draw_circle(Color::White);
    EPaper_display.flush();
    EPaper_display.wait_till_idle().unwrap();

    rtc.sleep_deep();
}