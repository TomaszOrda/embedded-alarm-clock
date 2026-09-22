#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use esp_backtrace as _;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::i2c::master::{Config as I2CConfig};
use esp_hal::{i2c, main};
use esp_hal::time::{Duration, Instant};
use log::info;
use embedded_alarm_clock::rtc::RTC;
use heapless::format;
use esp_hal::ram;

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();


const ALARMS_MAX_LENGTH: usize = 16;
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS: [AlarmTime;ALARMS_MAX_LENGTH] = [AlarmTime::placeholder(); ALARMS_MAX_LENGTH];
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS_LENGTH: usize = ALARMS_MAX_LENGTH+1;
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS_CHECKSUM: u16 = 0;

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
    
    let i2c = i2c::master::I2c::new(peripherals.I2C0,I2CConfig::default()).unwrap()
                                                                                             .with_scl(i2c_scl)
                                                                                             .with_sda(i2c_sda);
    let mut rtc: RTC = RTC::new(i2c, peripherals.LPWR, #[cfg(not(debug_assertions))] peripherals.GPIO5.into_pull_up_input().into());


    info!("High");
    led.set_high();
    info!("Current time {}", rtc.get_time_hh_mm().unwrap_or(format!("??:??").unwrap()));

    let delay_start = Instant::now();
    while delay_start.elapsed() < Duration::from_millis(1000) {}
    info!("Low");
    led.set_low();

    rtc.sleep_deep();
}


#[derive(PartialEq, Copy, Clone)]
struct AlarmTime{
    pub weekday: u8,
    pub hour: u8,
    pub minute: u8
}
unsafe impl esp_hal::Persistable for AlarmTime {}
impl AlarmTime{
    pub const fn placeholder()->Self{
        Self { weekday: 0, hour: 0, minute: 0 }
    }
    pub fn is_valid(&self)->bool{
        return (1_u8..=7_u8).contains(&self.weekday) 
            && (0_u8..60_u8).contains(&self.minute) 
            && (0_u8..24_u8).contains(&self.hour)
    }
}
struct AlarmTable{
}
impl AlarmTable{
    pub fn initialize(){
        if !AlarmTable::is_valid() || !AlarmTable::is_checksum_consistent(){
            AlarmTable::clear();
        }
    }
    fn is_valid()-> bool{
        unsafe {
            return ALARMS_LENGTH<=ALARMS_MAX_LENGTH && ALARMS[..ALARMS_LENGTH].iter().all(|alarm| alarm.is_valid())
        }
    }
    fn calculate_checksum() -> u16{
        unsafe{
            let mut bufor : [u8; 1+ ALARMS_MAX_LENGTH * 3] = [0_u8; 1+ALARMS_MAX_LENGTH * 3];
            bufor[0] = ALARMS_LENGTH as u8;
            let mut id = 1;
            for alarm in ALARMS[0..ALARMS_LENGTH].iter(){
                bufor[id] = alarm.weekday;
                bufor[id+1] = alarm.hour;
                bufor[id+2] = alarm.minute;
                id = id +3
            }
            return crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC).checksum(&bufor)
        }
    }
    fn is_checksum_consistent() -> bool{
        unsafe{
            ALARMS_CHECKSUM == AlarmTable::calculate_checksum()
        }
    }
    fn recalculate_checksum(){
        unsafe{
            ALARMS_CHECKSUM = AlarmTable::calculate_checksum();
        }
    }
    fn clear(){
        unsafe {
            ALARMS_LENGTH = 0;
            AlarmTable::recalculate_checksum();
        }
    }
    pub fn push_alarm(alarm: AlarmTime)->Option<()>{
        unsafe{
            if ALARMS_LENGTH == ALARMS_MAX_LENGTH{
                return None
            }
            ALARMS[ALARMS_LENGTH] = alarm;
            ALARMS_LENGTH = ALARMS_LENGTH + 1;
            AlarmTable::recalculate_checksum();
            Some(())
        }
    }
    pub fn contains(alarm: &AlarmTime)->bool{
        unsafe{
            return ALARMS[..ALARMS_LENGTH].contains(alarm)
        }
    }
}
