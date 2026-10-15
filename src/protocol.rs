use esp_hal::{Blocking, usb_serial_jtag::UsbSerialJtagRx};
use embassy_time::{Timer, Duration};
use log::info;

use crate::{alarm_table::{AlarmTable, AlarmTime}, rtc::RTC};


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
    rtc: RTC,
    alarm_table: &'static mut AlarmTable
}
impl AlarmClockPeripherals{ 
    pub fn new(rtc: RTC, alarm_table: &'static mut AlarmTable) -> Self{
        Self { rtc, alarm_table }
    }
    pub fn handle_command(&mut self, command: &str){
        let mut words = command.trim().split_whitespace();
        match words.next() {
            Some("get-time") => self.handle_get_time(),
            Some("set-time") => match words.next(){
                Some(time) => self.handle_set_time(time),
                None => info!("Wrong datetime format")
            }
            Some("get-alarms") => self.handle_get_alarms(),
            Some("set-alarm") => self.handle_set_alarm(&words.fold(heapless::String::<64>::new(), |mut acc, word| {acc.push_str(word).ok();acc.push_str(" ").ok(); acc})),
            Some("remove-alarm") => match words.next(){
                Some(id) => self.handle_remove_alarm(id),
                None => info!("Wrong datetime format")
            }
            Some("ping") => self.handle_ping(),
            Some("help") => self.handle_help(),
            None => (),
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
    pub fn handle_get_alarms(&self){
        info!("{}", self.alarm_table)
    }
    pub fn handle_set_alarm(&mut self, time: &str){
        let alarm: Option<AlarmTime> = AlarmTime::from_string(time);
        if alarm.is_none(){
            info!("Failed to parse alarm time: {}", time);
            return ();
        }
        match self.alarm_table.push_alarm(alarm.unwrap()){
            Some(_) => info!("Created alarm: {}", alarm.unwrap()),
            None => info!("Failed to push new alarm")
        }
    }
    pub fn handle_remove_alarm(&mut self, id_str: &str){
        let id: usize = match id_str.parse(){
            Ok(parsed_id) => parsed_id,
            Err(_e) => {info!("Failed to parse id {}", id_str); return ();}
        };
        match self.alarm_table.remove_alarm(id) {
            Some(()) => info!("Alarm at id {} removed", id),
            None => info!("Failed to remove alarm at id {}",id)
        }
    }
    pub fn handle_ping(&self){
        info!("pong")
    }
}