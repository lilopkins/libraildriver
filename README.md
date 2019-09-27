# Lib Rail Driver

Rust FFI bindings to the `RailDriver.dll` library.

These allow you to read and write data to or from Train Simulator 2020. Note
that this doesn't work with Train Sim World.

## Quick example

```rust
extern crate libraildriver;

fn main() {
    let context = libraildriver::Context::new();
    let speed = context.get_value(libraildriver::Value::Speedometer,
                  libraildriver::Kind::Current).expect("Failed to get value.");
    println!("The train's current speed is: {}", speed);
}
```
