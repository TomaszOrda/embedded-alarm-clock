

use embassy_time::Timer;
use esp_hal::gpio::{Output};
use esp_hal::time::{Duration, Instant};

pub struct Buzzer{
    pin: Output<'static>
}
impl Buzzer{
    pub fn new(pin: Output<'static>) -> Self{
        Buzzer{
            pin,
        }
    }
    pub async fn buzz(&mut self, duration_seconds: u8, frequency: f32){
        let half_period = embassy_time::Duration::from_micros((1_000_000.0 / frequency / 2.0) as u64);
        let beeping_end = Instant::now() + Duration::from_secs(duration_seconds as u64);
        while Instant::now() < beeping_end {
            self.pin.set_high();
            Timer::after(half_period).await;
            self.pin.set_low();
            Timer::after(half_period).await;
        }
    }
}