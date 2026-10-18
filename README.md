A basic alarm clock built using Rust + ESP32, connected to Wokwi simulator.

### Development

As a prerequisite you need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [Wokwi VScode extension](https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode) which requires logging in and generating temporary licence. [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is recommended.

To start you need to ``cargo build``. Then open `diagram.json` and run the simulation (or use `Ctrl+Shift+P` and choose `Wokwi: Start Simulation`).

To flash run ``cargo run --release`` while resetting the esp32. The device normally goes to sleep within seconds and is not flashable in that state.

For configuring a compiled clock the `.\cli` project is used. It is a separate rust project that needs to be `cargo run` from within the `.\cli` folder.

This project started using https://github.com/esp-rs/esp-generate/tree/main template.

### Usage 

Button is responsible for both LED (epaper has no backlit and can be hard to read in low light environment) and interrupting alarm. This double use is dictated by the number of available GPIOS.

Alarm can be disabled with a switch connected to the button. 

There is also a lightswitch, because the alarm-disabling-switch prevents the button from turning on the LED.

CLI establishes pairing with a usb connected clock. Commands like `help`, `set-time`, `set-alarm` can be used. A clock reset is needed after finished configuration to come back to the normal non-pairing mode.

### Hardware

This board can use either DS3231 or DS1307. Wokwi only supports DS1307, but the whole project was built with DS3231 in mind.

DS3231 has an onboard LED that drains a substantial amount of power compared to the rest of the device. [A solution](https://forum.arduino.cc/t/safe-to-remove-power-led-from-ds3231-clock-module/217686) is to desolder `102` resistor right next to it.

Buzzer can be replaced with a coin cell vibration motor for a more pleasant (lower frequency) alarm experience. Wokwi does not support such hardware, thus a simple buzzer was used for simulation. However such motors usually require ~70mA current which is way more than GPIO can output. It will work using 40mA mode through a small resistor to limit pin wear — GPIOs should not be used too much above 20mA. A better solution is to use a N-MOSFET, basically any long-legged version like 2N7000 should work fine.

Because of the limited number of pins we can not really disable the alarm without disabling the button$\rightarrow$LED interaction. That is why the second switch turns the LED — providing a lightswitch working regardless of if the button/alarm is disabled or not. Alarm disabling switch can be replaced with a wire between button 2.r and LED resistor. The lightswitch can be replaced with wires — same as their default state — without much impact on the whole system.

The two switches and one button system might seem unreasonable. However, I began to like its character a lot. More so because of the minimalistic controls it should not be misleading. In the end each input has a specific task it performs and limited side effects.

### Power consumption

Assuming one alarm per day runing for 10 seconds, similar amount for LED, 2s cycle every minute with 1.5s epaper refresh I assume following (thise just are rough estimates):

| Device   | Power active | Power standby | Daily energy consumption |
| -------- | ------- | ------- | ------- | 
| Coin cell vibrator | $50mA$ | $0A$ | $0.139mAh \approx 50mA *10s/2 $|
| EPD | $10mA$ | $5\mu A$  | $6.120mAh \approx 10mA *1440 *1.5s + 5\mu A *24h$|
| ESP32C3 | $15mA$ | $45\mu A$  | $13.080mAh \approx 15mA *1440 *2s + 45\mu A *24h$|
| DS3231 | $80\mu A$ | $80\mu A$  | $1.920mAh \approx 80\mu A *24h$|
| LED | $1mA$ | $0 A$  | $0.003mAh \approx 1mA *10s$|
| **Total** | $76mA$ | $105\mu A$ | $21.262mAh$ | 

Which should last for more than 3 months using $2200mAh$ LiPo battery. This was not yet tested. I expect the usable batter charge to be considerably less.

### Known issues

The epaper display currently can only do a full refresh. Through something that is likely a library issue, partial refresh works just as a full refresh. I have not yet found a solution to this. It is important because of the annoying flickering during full flush and wake-time amount (power consumption).

