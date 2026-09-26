A basic alarm clock built using Rust + ESP32, connected to Wokwi simulator.

As a prerequisite you need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [Wokwi VScode extension](https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode) which requires logging in and generating temporary licence. [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is recommended.

To start you need to ``cargo build``. Then open `diagram.json` and run the simulation (or use `Ctrl+Shift+P` and choose `Wokwi: Start Simulation`).

This project started using https://github.com/esp-rs/esp-generate/tree/main template.