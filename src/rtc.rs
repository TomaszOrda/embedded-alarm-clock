
use ds323x::{DateTimeAccess, DayAlarm1, Ds323x, NaiveDateTime, NaiveTime, Rtcc, Timelike};
use heapless::{String, format};
use esp_hal::{peripherals::LPWR, rtc_cntl::Rtc};
#[cfg(not(debug_assertions))]
use esp_hal::{peripherals::LPWR, rtc_cntl::sleep::WakeupLevel};

type I2c = esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>;

pub struct RTC{
    rtc: Ds323x<ds323x::interface::I2cInterface<I2c>, ds323x::ic::DS3231>,
    mcu_rtc: Rtc<'static>,
    #[cfg(not(debug_assertions))]
    wake_pin: esp_hal::gpio::AnyPin<'static>
}
impl RTC{
    pub fn new(i2c: I2c, lpwr: LPWR<'static>, #[cfg(not(debug_assertions))] wake_pin: esp_hal::gpio::AnyPin<'static>) ->Self{
        let mut this = Self{
            rtc: Ds323x::new_ds3231(i2c),
            mcu_rtc: Rtc::new(lpwr),
            #[cfg(not(debug_assertions))]
            wake_pin
        };
        this.set_alarm_every_minute();
        this
    }
    pub fn get_time(&mut self) -> Option<NaiveTime>{
        return match self.rtc.time(){
            Ok(time) => Some(time),
            Err(_e) => None
        }
    }
    pub fn get_date_time(&mut self) -> Option<NaiveDateTime>{
        return match self.rtc.datetime(){
            Ok(time) => Some(time),
            Err(_e) => None
        }
    }
    pub fn get_time_hh_mm(&mut self) ->Option<String<5>>{
        return match self.rtc.time(){
            Ok(time) => Some(format!("{:02}:{:02}",time.hour(), time.minute()).unwrap()),
            Err(_e) => None
        }
    }
    #[cfg(not(debug_assertions))]
    pub fn sleep_deep(&mut self) -> !{
        self.rtc.clear_alarm1_matched_flag().unwrap();
        self.mcu_rtc.sleep_deep(&[&RtcioWakeupSource::new(&mut [(&mut self.wake_pin, WakeupLevel::Low)])])
    }
   #[cfg(debug_assertions)]
    pub fn sleep_deep(&mut self) -> !{
        use esp_hal::rtc_cntl::sleep::TimerWakeupSource;
        use core::time::Duration;

        let seconds_left_to_a_full_minute = (61 - self.rtc.seconds().unwrap()) as u64;
        self.mcu_rtc.sleep_deep(&[&TimerWakeupSource::new(Duration::from_secs(seconds_left_to_a_full_minute))])
    }
    fn set_alarm_every_minute(&mut self) {
        let alarm = DayAlarm1{
            day: 1, //Ignored
            hour: ds323x::Hours::H24(0),//Ignored
            minute: 0,//Ignored
            second: 0
        };
        self.rtc.set_alarm1_day(alarm, ds323x::Alarm1Matching::SecondsMatch).unwrap();
    }
}