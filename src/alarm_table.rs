use core::fmt;

use ds323x::{Datelike, NaiveDateTime, Timelike};
const ALARMS_MAX_LENGTH: u8 = 16;
const CRC16: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);

#[derive(Default, PartialEq, Copy, Clone)]
pub struct AlarmTime{
    //Adjusting the values can introduce a padding and make the struct non esp_hal::Persistable
    pub weekday: u8,
    pub hour: u8,
    pub minute: u8
}
unsafe impl esp_hal::Persistable for AlarmTime {}
impl AlarmTime{
    pub fn is_valid(&self)->bool{
        return (0_u8..7_u8).contains(&self.weekday) 
            && (0_u8..60_u8).contains(&self.minute) 
            && (0_u8..24_u8).contains(&self.hour)
    }
    pub const fn zeroed() -> Self{
        Self{
            weekday: 0,
            hour: 0,
            minute: 0
        }
    }
    // up to 9 character string
    fn weekday_to_string(weekday:u8) -> &'static str{
        match weekday{
            0 => "Monday",
            1 => "Tuesday",
            2 => "Wednesday",
            3 => "Thursday",
            4 => "Friday",
            5 => "Saturday",
            6 => "Sunday",
            _ => "Unknown"
        }
    }
    pub fn string_to_weekday(weekday_string:&str) -> Option<u8>{
        if weekday_string.len()<3{
            return None
        }
        let mut heapless_weekday = heapless::String::<64>::try_from(weekday_string).ok()?;
        heapless_weekday.make_ascii_lowercase();
        match &heapless_weekday.as_bytes()[0..3]{
            b"mon" => Some(0),
            b"tue" => Some(1),
            b"wed" => Some(2),
            b"thu" => Some(3),
            b"fri" => Some(4),
            b"sat" => Some(5),
            b"sun" => Some(6),
            _ => None
        }
    }
    pub fn from_string(time:&str) -> Option<Self>{
        let mut parts = time.trim().split_whitespace();
        let weekday = AlarmTime::string_to_weekday(parts.next()?)?;
        let mut hour_minute =  parts.next()?.split(":");
        let hour:u8 = hour_minute.next()?.parse().ok()?;
        let minute:u8 = hour_minute.next()?.parse().ok()?;
        let alarm = Self{
            weekday: weekday,
            hour: hour,
            minute: minute
        };
        if !alarm.is_valid(){
            return None
        }
        Some(alarm)
    }
}
impl From<NaiveDateTime> for AlarmTime {
    fn from(time: NaiveDateTime) -> Self {
        Self {
            weekday: time.weekday() as u8,
            hour: time.hour() as u8,
            minute: time.minute() as u8,
        }
    }
}
impl From<&NaiveDateTime> for AlarmTime {
    fn from(time: &NaiveDateTime) -> Self {
        Self {
            weekday: time.weekday() as u8,
            hour: time.hour() as u8,
            minute: time.minute() as u8,
        }
    }
}
impl fmt::Display for AlarmTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:9} — {:02}:{:02}",AlarmTime::weekday_to_string(self.weekday),self.hour,self.minute)
    }
}
pub struct AlarmTable{
    //Adjusting the values can introduce a padding and make the struct non esp_hal::Persistable
    pub table: [AlarmTime;ALARMS_MAX_LENGTH as usize],
    table_length: u8,
    checksum: u16
}
unsafe impl esp_hal::Persistable for AlarmTable {}
impl AlarmTable{
    pub const fn new() -> Self{
        Self{
            table: [AlarmTime::zeroed(); ALARMS_MAX_LENGTH as usize],
            table_length: 0,
            checksum: 0
        }
    }
    pub fn is_consistent(&self)->bool{
        self.is_valid() && self.is_checksum_consistent()
    }
    fn is_valid(&self)-> bool{
        return self.table_length<=ALARMS_MAX_LENGTH && self.table[..self.table_length as usize].iter().all(|alarm| alarm.is_valid())
    }
    fn calculate_checksum(&self) -> u16{
        let mut digest = CRC16.digest();
        digest.update(&[self.table_length]);
        self.table.iter().for_each(
            |alarm| digest.update(&[alarm.weekday, alarm.hour, alarm.minute]) );
        digest.finalize()
    }
    fn is_checksum_consistent(&self) -> bool{
        self.checksum == self.calculate_checksum()
    }
    fn recalculate_checksum(&mut self){
        self.checksum = self.calculate_checksum();
    }
    pub fn clear(&mut self){
        self.table_length = 0;
        self.recalculate_checksum();
    }
    pub fn remove_alarm(&mut self, id: usize) -> Option<()>{
        if id >= self.table_length as usize{
            return None
        }
        for alarm_id in (id+1)..self.table_length as usize{
            self.table[alarm_id-1] = self.table[alarm_id]
        }
        self.table_length = self.table_length - 1;
        self.recalculate_checksum();
        Some(())
    }
    pub fn push_alarm(&mut self, alarm: AlarmTime)->Option<()>{
        if self.table_length == ALARMS_MAX_LENGTH{
            return None
        }
        self.table[self.table_length as usize] = alarm;
        self.table_length = self.table_length + 1;
        self.recalculate_checksum();
        Some(())
    }
    pub fn contains(&self, alarm: &AlarmTime)->bool{
        return self.table[..self.table_length as usize].contains(alarm)
    }
    pub fn contains_date_time(&self, alarm: &NaiveDateTime)->bool{
        return self.table[..self.table_length as usize].contains(&alarm.into())
    }
}
impl fmt::Display for AlarmTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (id,alarm) in self.table.iter().take(self.table_length as usize).enumerate(){
            write!(f,"{}: {}\n", id, alarm)?;
        }
        Ok(())
    }
}