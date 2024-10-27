The code in this directory is for testing the GridTrust interfacing device, which consists of a SEL751 relay connected to a computer using a USB-to-RS232 connection.


This code will write `update.txt` to serial port `/dev/ttyUSB0`. `update.txt` contains commands that will be executed on the relay.

**Compilation and running:**

`cargo run` in the same directory as `Cargo.toml`
