const ALARMS_MAX_LENGTH: u8 = 16;

#[derive(PartialEq, Copy, Clone)]
pub struct AlarmTime{
    //Adjusting the values can introduce a padding and make the struct non esp_hal::Persistable
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
    //Adjusting the values can introduce a padding and make the struct non esp_hal::Persistable
    table: [AlarmTime;ALARMS_MAX_LENGTH as usize],
    table_length: u8,
    checksum: u16
}
unsafe impl esp_hal::Persistable for AlarmTable {}
impl AlarmTable{
    pub const fn new() -> Self{
        Self{
            table: [AlarmTime::placeholder(); ALARMS_MAX_LENGTH as usize],
            table_length: 0,
            checksum: 0
        }
    }
    pub fn initialize(&mut self){
        if !self.is_valid() || !self.is_checksum_consistent(){
            self.clear();
        }
    }
    fn is_valid(&self)-> bool{
        return self.table_length<=ALARMS_MAX_LENGTH && self.table[..self.table_length as usize].iter().all(|alarm| alarm.is_valid())
    }
    fn calculate_checksum(&self) -> u16{
        let mut bufor : [u8; 1+ ALARMS_MAX_LENGTH as usize * 3] = [0_u8; 1+ALARMS_MAX_LENGTH as usize * 3];
        bufor[0] = self.table_length as u8;
        let mut id = 1;
        for alarm in self.table[..self.table_length as usize].iter(){
            bufor[id] = alarm.weekday;
            bufor[id+1] = alarm.hour;
            bufor[id+2] = alarm.minute;
            id = id +3
        }
        return crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC).checksum(&bufor)
    }
    fn is_checksum_consistent(&self) -> bool{
        self.checksum == self.calculate_checksum()
    }
    fn recalculate_checksum(&mut self){
        self.checksum = self.calculate_checksum();
    }
    fn clear(&mut self){
        self.table_length = 0;
        self.recalculate_checksum();
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
}
