This is a basic template for any kind of Rust+STM32 project with connected Wokwi simulator. As it stands it should build and run just fine. Serves as a starting point towards more complicated projects.

To run you need to ``cargo build release``. Then open `diagram.json` and run the simulation.

As a prerequisite you need to install [Rust](https://doc.rust-lang.org/book/ch01-01-installation.html) and [Wokwi VScode extension](https://marketplace.visualstudio.com/items?itemName=Wokwi.wokwi-vscode) which requires logging in and generating temporary licence. [Rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) is recommended.

The template is based on [Uri Shaked example project](https://wokwi.com/projects/367244067477216257) and some very very basic generated code.