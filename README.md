# Project Title

A crazy fast app to process 1000 page pdf to text in 1 second!! Deployed:https://fastpdftext.netlify.app

TLDR: How I built it
-> Rust + WebAssembly for blazing-fast PDF processing
-> Simple drag-drop UI with vanilla JavaScript
-> Zero backend - all runs in browser
-> Privacy-first: no data storage

## Description

This project is a WebAssembly application built using Rust and JavaScript. It leverages various libraries to provide functionality.

## Installation

1. Ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.
2. Install the required dependencies by running:
   ```bash
   cargo build
   ```

## Running the Project

To run the project, use the following command:

```bash
cargo run
```

## Dependencies

- `wasm-bindgen`
- `js-sys`
- `web-sys`
- `serde`
- `serde_json`
- `serde-wasm-bindgen`
- `thiserror`
- `lopdf`
- `wasm-bindgen-futures`
- `console_error_panic_hook`
- `wasm-bindgen-test`

## License

This project is licensed under the MIT License.
