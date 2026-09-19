
use ds323x::{Ds323x, NaiveTime, Rtcc, Timelike};
use heapless::{String, format};


type I2c = esp_hal::i2c::master::I2c<'static, esp_hal::Blocking>;

pub struct RTC{
    rtc: Ds323x<ds323x::interface::I2cInterface<I2c>, ds323x::ic::DS3231>
}
impl RTC{
    pub fn new(i2c: I2c) ->Self{
        Self{
            rtc: Ds323x::new_ds3231(i2c)
        }
    }
    pub fn get_time(&mut self) -> Option<NaiveTime>{
        return match self.rtc.time(){
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
}