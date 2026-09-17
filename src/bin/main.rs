#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]


use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::Circle;
use embedded_graphics::primitives::PrimitiveStyle;
use embedded_graphics::prelude::*;
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::epd2in9_v2::*;
use epd_waveshare::prelude::*;
use esp_backtrace as _;
use esp_hal::Blocking;
use esp_hal::clock::CpuClock;
use esp_hal::gpio::Level::{self};
use esp_hal::gpio::{Input, InputConfig, Output, OutputConfig};
use esp_hal::main;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::{Duration, Instant, Rate};
use log::info;
use esp_hal::delay::Delay;


// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[main]
fn main() -> ! {
    // generator version: 1.3.0
    // generator parameters: --chip esp32c3 -o esp32c3-mini-1 -o log -o esp-backtrace -o wokwi -o vscode -o stable-x86_64-pc-windows-msvc

    esp_println::logger::init_logger_from_env();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    let spi_mosi = peripherals.GPIO10;
    let spi_sck = peripherals.GPIO8;
    let _u0rxd = peripherals.GPIO20;
    let _i2c_sda = peripherals.GPIO6;
    let _i2c_scl = peripherals.GPIO7;
    // The following pins are used to bootstrap the chip. They are available
    // for use, but check the datasheet of the module for more information on them.
    // - GPIO2
    // - GPIO8
    // - GPIO9
    // These GPIO pins are in use by some feature of the module and should not be used.
    let _ = peripherals.GPIO11;
    let _ = peripherals.GPIO12;
    let _ = peripherals.GPIO13;
    let _ = peripherals.GPIO14;
    let _ = peripherals.GPIO15;
    let _ = peripherals.GPIO16;
    let _ = peripherals.GPIO17;

    let cs = Output::new(peripherals.GPIO9, Level::High, OutputConfig::default());
    let dc = Output::new(peripherals.GPIO3, Level::High, OutputConfig::default());
    let rst = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
    let busy = Input::new(peripherals.GPIO21, InputConfig::default());

    let mut led = Output::new(peripherals.GPIO2,esp_hal::gpio::Level::Low, OutputConfig::default());

    let spi = Spi::new(peripherals.SPI2, 
                       Config::default().with_mode(Mode::_0)
                                        .with_frequency(Rate::from_mhz(4)))
                                        .unwrap()
                                        .with_sck(spi_sck)
                                        .with_mosi(spi_mosi);
    let mut EPaper_display = EPaperDisplay::new(spi, cs, busy, dc, rst);
    EPaper_display.wait_till_idle().unwrap();

    info!("frame displayed");
    loop {
        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("High");
        led.set_high();
        EPaper_display.draw_circle(Color::White);

        let delay_start = Instant::now();
        while delay_start.elapsed() < Duration::from_millis(1000) {}
        info!("Low");
        led.set_low();
        EPaper_display.draw_circle(Color::Black);

    }

}
type SPI = Spi<'static, Blocking>;
type CS = Output<'static>;
type BUSY = Input<'static>;
type DC = Output<'static>;
type RST = Output<'static>;
type SPIDEV = ExclusiveDevice<SPI, CS, Delay>;

struct EPaperDisplay
{
    spi_dev: SPIDEV,
    epd2in9: Epd2in9<SPIDEV, BUSY, DC, RST, Delay>,
    display: Display2in9,
    delay: Delay
}
impl EPaperDisplay
{
    pub fn new(spi: SPI, cs: CS, busy: BUSY, dc: DC, rst: RST) -> Self{
        let mut delay = Delay::new();
        let mut spi_dev = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
        let mut this = Self {
            epd2in9 : Epd2in9::new(&mut spi_dev, busy, dc, rst, &mut delay,None).unwrap(),
            delay,
            spi_dev,
            display : Display2in9::default()
        };
        this.display.clear(Color::Black);
        this.epd2in9.update_old_frame(&mut this.spi_dev, &this.display.buffer(), &mut this.delay).unwrap();
        this.epd2in9.display_frame(&mut this.spi_dev, &mut this.delay).unwrap();
        this
    }
    pub fn draw_circle(&mut self, color: Color){
        Circle::new(Point::new(10, 10), 20)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(&mut self.display) // or .draw(&mut epd.display)
        .unwrap();
        self.push_quick();
    }
    fn push_quick(&mut self){
        self.epd2in9.update_new_frame(&mut self.spi_dev, &self.display.buffer(), &mut self.delay).unwrap();
        self.epd2in9.display_new_frame(&mut self.spi_dev, &mut self.delay).unwrap();
    }
    pub fn wait_till_idle(&mut self) -> Result<(), embedded_hal_bus::spi::DeviceError<esp_hal::spi::Error, core::convert::Infallible>>{
        return self.epd2in9.wait_until_idle(&mut self.spi_dev, &mut self.delay)
    }
}