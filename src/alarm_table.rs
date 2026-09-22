use esp_hal::ram;

const ALARMS_MAX_LENGTH: usize = 16;
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS: [AlarmTime;ALARMS_MAX_LENGTH] = [AlarmTime::placeholder(); ALARMS_MAX_LENGTH];
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS_LENGTH: usize = ALARMS_MAX_LENGTH+1;
#[ram(unstable(rtc_fast, persistent))]
static mut ALARMS_CHECKSUM: u16 = 0;


#[derive(PartialEq, Copy, Clone)]
pub struct AlarmTime{
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
pub struct AlarmTable{
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
