#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]


use ds323x::{NaiveTime, Timelike};
use esp_backtrace as _;
use embassy_time::{Timer, Duration};
use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;
use esp_hal::i2c::master::{Config as I2CConfig};
use esp_hal::rtc_cntl::wakeup_cause;
use esp_hal::i2c;
use esp_hal::time::Rate;
use esp_hal::gpio::{Input, InputConfig, Level::{self}, Output, OutputConfig};
use esp_hal::spi::{Mode, master::{Config, Spi}};
use log::info;

use embedded_alarm_clock::epaper::EPaperDisplay;
use embedded_alarm_clock::buzzer::Buzzer;
use embedded_alarm_clock::rtc::RTC;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o esp32c3-mini-1 -o log -o esp-backtrace -o wokwi -o vscode -o stable-x86_64-pc-windows-msvc

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let software_interrupt = esp_hal::interrupt::software::SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);
    esp_rtos::start(timg0.timer0, software_interrupt.software_interrupt0);

    let i2c_sda = peripherals.GPIO6;
    let i2c_scl = peripherals.GPIO7;
    let spi_mosi = peripherals.GPIO10;
    let spi_sck = peripherals.GPIO8;
    let u0rxd = peripherals.GPIO20;
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

    let woke_from_sleep: bool = match wakeup_cause(){
        esp_hal::system::SleepSource::Timer => true,
        esp_hal::system::SleepSource::Ext1 => true,
        _ => false
    };

    let cs = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
    let busy = Input::new(peripherals.GPIO3, InputConfig::default());

    let spi = Spi::new(peripherals.SPI2, 
                       Config::default().with_mode(Mode::_0)
                                        .with_frequency(Rate::from_mhz(4)))
                                        .unwrap()
                                        .with_sck(spi_sck)
                                        .with_mosi(spi_mosi);
    let mut epaper_display = EPaperDisplay::new(spi, cs, busy, dc, rst, woke_from_sleep);
    epaper_display.wait_till_idle().unwrap();
    info!("Display initialized");

    //Note that we use u0rxd to save one pin for button
    let button: Input<'_> = Input::new(u0rxd, InputConfig::default().with_pull(esp_hal::gpio::Pull::None));
    let buzzer: Buzzer = Buzzer::new(Output::new(peripherals.GPIO4, esp_hal::gpio::Level::Low, OutputConfig::default()));

    if rtc.get_time().unwrap().minute()%1 == 0 {
        spawner.spawn(update_display_task(epaper_display, rtc.get_time().unwrap()).unwrap());
        alarm(buzzer, 10, 0.2, button).await;
    } else{
        update_display(&mut epaper_display, rtc.get_time().unwrap());
    }
    

    rtc.sleep_deep();
}

fn update_display(epaper_display: &mut EPaperDisplay, time: NaiveTime) {
    let current_time_string = RTC::format_time_hh_mm(&time);
    info!("Current time {}", current_time_string);
    epaper_display.draw_text(&current_time_string);
    epaper_display.flush();
    epaper_display.wait_till_idle().unwrap();
}

#[embassy_executor::task]
async fn update_display_task(mut epaper_display: EPaperDisplay, time: NaiveTime) {
    let mut updated_time = time;
    loop {
        update_display(&mut epaper_display, updated_time);
        Timer::after(Duration::from_secs(60)).await;
        updated_time = updated_time + core::time::Duration::from_secs(60);
    }
}

async fn alarm(mut buzzer: Buzzer, duration_seconds: u8, frequency: f32, mut interupt_button: Input<'static>) {
    info!("Buzzing");
    buzzer.buzz(duration_seconds, frequency, &mut interupt_button).await;
    info!("Buzzing stopped");
}
