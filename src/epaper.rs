use embedded_graphics::draw_target::DrawTarget;
use embedded_graphics::geometry::Point;
use embedded_graphics::primitives::{Circle, PrimitiveStyle};
use embedded_graphics::prelude::*;
use u8g2_fonts::fonts::u8g2_font_logisoso92_tn;
use u8g2_fonts::{FontRenderer};
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::{epd2in9_v2::*, prelude::*};
use esp_backtrace as _;
use esp_hal::Blocking;
use esp_hal::gpio::{Input, Output};
use esp_hal::spi::master::Spi;
use esp_hal::delay::Delay;


type SPI = Spi<'static, Blocking>;
type CS = Output<'static>;
type BUSY = Input<'static>;
type DC = Output<'static>;
type RST = Output<'static>;
type SPIDEV = ExclusiveDevice<SPI, CS, Delay>;


pub struct EPaperDisplay
{
    spi_dev: SPIDEV,
    epd2in9: Epd2in9<SPIDEV, BUSY, DC, RST, Delay>,
    display: Display2in9,
    delay: Delay,
    partial_flush_counter: u8
}
impl EPaperDisplay
{
    const PARTIAL_FLUSH_LIMIT: u8 = 31;
    pub fn new(spi: SPI, cs: CS, busy: BUSY, dc: DC, rst: RST) -> Self{
        let mut delay = Delay::new();
        let mut spi_dev = ExclusiveDevice::new(spi, cs, Delay::new()).unwrap();
        let mut this = Self {
            epd2in9 : Epd2in9::new(&mut spi_dev, busy, dc, rst, &mut delay,None).unwrap(),
            delay,
            spi_dev,
            display : Display2in9::default(),
            partial_flush_counter : 0
        };
        this.display.clear(Color::White);
        this.display.set_rotation(DisplayRotation::Rotate90);
        this.flush_full();
        this
    }
    pub fn draw_circle(&mut self, color: Color){
        Circle::new(Point::new(10, 10), 20)
        .into_styled(PrimitiveStyle::with_fill(color))
        .draw(&mut self.display) // or .draw(&mut epd.display)
        .unwrap();
    }
    pub fn draw_text(&mut self, text:&str){
        let renderer : FontRenderer = FontRenderer::new::<u8g2_font_logisoso92_tn>();
        renderer.render_aligned(
            text,
            self.display.bounding_box().center(),
            u8g2_fonts::types::VerticalPosition::Center,
            u8g2_fonts::types::HorizontalAlignment::Center,
            u8g2_fonts::types::FontColor::Transparent(Color::Black),
            &mut self.display
        ).unwrap();
    }
    pub fn flush(&mut self){
        if self.partial_flush_limit_reached(){
            self.flush_full();
        }else{
            self.flush_quick();
        }
    }
    fn flush_quick(&mut self){
        self.epd2in9.update_new_frame(&mut self.spi_dev, &self.display.buffer(), &mut self.delay).unwrap();
        self.epd2in9.display_new_frame(&mut self.spi_dev, &mut self.delay).unwrap();
        self.partial_flush_counter+=1;
    }
    fn flush_full(&mut self){    
        self.epd2in9.update_old_frame(&mut self.spi_dev, &self.display.buffer(), &mut self.delay).unwrap();
        self.epd2in9.display_frame(&mut self.spi_dev, &mut self.delay).unwrap();
        self.partial_flush_counter = 0;
    }
    pub fn wait_till_idle(&mut self) -> Result<(), embedded_hal_bus::spi::DeviceError<esp_hal::spi::Error, core::convert::Infallible>>{
        return self.epd2in9.wait_until_idle(&mut self.spi_dev, &mut self.delay)
    }
    fn partial_flush_limit_reached(&self) -> bool{
        return self.partial_flush_counter >= Self::PARTIAL_FLUSH_LIMIT;
    }
}