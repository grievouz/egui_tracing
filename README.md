# egui_tracing

> **Note**: This library is under active development. Breaking changes may occur in future releases.

[![Crates.io](https://img.shields.io/crates/v/egui_tracing)](https://crates.io/crates/egui_tracing)
[![Documentation](https://docs.rs/egui_tracing/badge.svg)](https://docs.rs/egui_tracing)
[![Apache](https://img.shields.io/badge/license-Unlicense-blue.svg)](https://github.com/grievouz/egui_tracing/blob/main/LICENSE)


`egui_tracing` is a Rust library that integrates tracing and logging capabilities with the [egui](https://github.com/emilk/egui) immediate mode graphical user interface library. It supports both the [tracing](https://crates.io/crates/tracing) and [log](https://crates.io/crates/log) crates for efficient and flexible event collection and visualization.

[Click to view the web demo](https://grievouz.github.io/egui_tracing/)

## Installation

Add `egui_tracing` to your `Cargo.toml` dependencies:

```toml
[dependencies]
egui_tracing = "0.2.2"
```

## Example

![Preview Image](./docs/preview.png)

To run the example application, use the following command:

```sh
cargo run --bin example-eframe
```

## Contributing

Contributions to `egui_tracing` are welcome! Please open an issue or submit a pull request on the GitHub repository.

## License

`egui_tracing` is released under the [Unlicense](LICENSE.md). This means the code is in the public domain, and you can use, modify, and distribute it without any restrictions. For more information, please see the [Unlicense website](https://unlicense.org/).
