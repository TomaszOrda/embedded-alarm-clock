use esp_hal::{Blocking, usb_serial_jtag::UsbSerialJtagRx};
use embassy_time::{Timer, Duration};
use log::info;

use crate::rtc::RTC;


pub struct SerialConsole{
    controller: AlarmClockPeripherals,
    rx: UsbSerialJtagRx<'static, Blocking>
}
impl SerialConsole{
    const COMMUNICATION_BUFFER_SIZE: usize = 64;
    pub fn new(controller: AlarmClockPeripherals, rx: UsbSerialJtagRx<'static, Blocking>)->Self{
        Self{
            controller,
            rx
        }
    }
    pub async fn start_communication(&mut self) -> !{
        let mut buf = heapless::String::<{SerialConsole::COMMUNICATION_BUFFER_SIZE}>::new();
        loop {
            match self.rx.read_byte() {
                Ok(b) => {
                    if b == b'\n' || b == b'\r' {
                        self.controller.handle_command(&buf);
                        buf.clear();
                    }else{
                        if let Err(_) = buf.push(b as char){
                            info!("Command too long");
                            buf.clear();
                            while let Ok(b) = self.rx.read_byte(){
                                if b == b'\n' || b == b'\r'{
                                    break
                                }
                            };
                            continue;
                        }
                    }
                }
                _ => Timer::after(Duration::from_millis(50)).await,  // fine to be lazy now
            }
        }
    }
}

pub struct AlarmClockPeripherals{
    rtc: RTC
}
impl AlarmClockPeripherals{ 
    pub fn new(rtc: RTC) -> Self{
        Self { rtc }
    }
    pub fn handle_command(&mut self, command: &str){
        let mut words = command.trim().split_whitespace();
        match words.next() {
            Some("get-time") => self.handle_get_time(),
            Some("set-time") => match words.next(){
                Some(time) => self.handle_set_time(time),
                None => info!("Wrong datetime format")
            }
            Some("help") => self.handle_help(),
            Some("") | None => (),
            Some(other) => info!("ERR unknown: {}", other)
        }
    }
    pub fn handle_get_time(&mut self){
        match self.rtc.get_time_hh_mm() {
            Some(t) => info!("TIME {}", t.as_str()),
            None => info!("ERR no rtc"),
        }
    }
    pub fn handle_set_time(&mut self, time: &str){
        self.rtc.set_date_time(time);
        info!("Setting time to {}", time);
    }
    pub fn handle_help(&self){
        info!("CMDS time buzz")
    }
}