

use embassy_futures::select::{select, Either};
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
    pub async fn buzz(&mut self, duration_seconds: u8, frequency: f32, interrupt_button: &mut Input<'_>){
        let half_period = embassy_time::Duration::from_micros((1_000_000.0 / frequency / 2.0) as u64);
        let beeping_end = Instant::now() + Duration::from_secs(duration_seconds as u64);
        while Instant::now() < beeping_end {
            if interrupt_button.is_low(){
                break;
            }
            self.pin.set_high();
            match select(Timer::after(half_period), interrupt_button.wait_for_falling_edge()).await{
                Either::First(_) => {},
                Either::Second(_) => break
            };
            self.pin.set_low();
            match select(Timer::after(half_period), interrupt_button.wait_for_falling_edge()).await{
                Either::First(_) => {},
                Either::Second(_) => break
            };
        }
        self.pin.set_low();
    }
}