A basic alarm clock built using Rust + ESP32, connected to Wokwi simulator.

### Development

As a prerequisite you need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [Wokwi VScode extension](https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode) which requires logging in and generating temporary licence. [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is recommended.

To start you need to ``cargo build``. Then open `diagram.json` and run the simulation (or use `Ctrl+Shift+P` and choose `Wokwi: Start Simulation`).

To flash run ``cargo run --release`` while reseting the esp32. The device normally goes to sleep within seconds and is not flashable in that state.

For configuring a compiled clock the `.\cli` project is used. It is a separate rust project that needs to be `cargo run` from within the `.\cli` folder.

This project started using https://github.com/esp-rs/esp-generate/tree/main template.

### Usage

For configuring a compiled clock the `.\cli` project is used. It is a separate rust project that needs to be `cargo run` from within the `.\cli` folder. It establishes pairing with a usb connected clock. Commands like `help`, `set-time`, `set-alarm` can be used. A clock reset is needed after finished configuration to come back to the normal non-pairing mode.
