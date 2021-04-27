# Minecraft Pi Edition API for Rust

This project ports the MCPI API to rust!

## How to install

Simply install this by adding `mcpi_api = "0.2.2"` to your Cargo.toml

## Examples

Hello world!
```rust
use mcpi_api::create;
let mut  mc = create("localhost:4711");
mc.post_to_chat("Hello World!")
```
