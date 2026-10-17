A basic alarm clock built using Rust + ESP32, connected to Wokwi simulator.

As a prerequisite you need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [Wokwi VScode extension](https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode) which requires logging in and generating temporary licence. [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is recommended.

To start you need to ``cargo build``. Then open `diagram.json` and run the simulation (or use `Ctrl+Shift+P` and choose `Wokwi: Start Simulation`).

This project started using https://github.com/esp-rs/esp-generate/tree/main template.

For configuring compiled clock the `.\cli` project is used. It is a separate rust project that needs to be `cargo run` from within `.\cli` folder. It establishes pairing with usb connected clock. Commands like `help`, `set-time`, `set-alarm` can be used. A clock reset is needed after finished configuration to come back to the normal non-pairing mode.

Alarm can be disabled with a switch. Because of the limited number of pins we can not really disable the alarm without disabling the button$\rightarrow$LED interaction. That is why the second switch turns the LED — providing a lightswitch working regardless of if the button/alarm is disabled or not. Alarm disabling switch can be replaced with a wire between button 2.r and LED rezistor. The lightswitch can be removed completely without much impact on the whole system.