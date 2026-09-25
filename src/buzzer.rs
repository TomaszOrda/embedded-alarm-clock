

use embassy_time::Timer;
use esp_hal::gpio::{Output, Input};
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
    pub async fn buzz(&mut self, duration_seconds: u8, frequency: f32, interrupt_button: &Input<'_>){
        let half_period = embassy_time::Duration::from_micros((1_000_000.0 / frequency / 2.0) as u64);
        let beeping_end = Instant::now() + Duration::from_secs(duration_seconds as u64);
        while Instant::now() < beeping_end {
            if interrupt_button.is_low(){
                break;
            }
            self.pin.set_high();
            Timer::after(half_period).await;
            self.pin.set_low();
            if interrupt_button.is_low(){
                break;
            }
            Timer::after(half_period).await;
        }
        self.pin.set_low();
    }
}