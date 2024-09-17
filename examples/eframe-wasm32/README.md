# Example: eframe-wasm32

This example demonstrates how to use [egui_tracing](https://github.com/grievouz/egui_tracing) with [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) in a web enviroment using [WebAssembly](https://webassembly.org/).

## Usage

To use this example, make sure you have [just](https://github.com/casey/just), [wasm-pack](https://github.com/rustwasm/wasm-pack), and [miniserve](https://github.com/svenstaro/miniserve) installed on your system.

```sh
just build && just serve
```