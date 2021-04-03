# Minecraft Pi Edition API for Rust

This project ports the MCPI API to rust!

## Examples

Hello world!
```rust
use mcpi_api::create;
let mut  mc = create("localhost:4711");
mc.post_to_chat("Hello World!")
```
