The code in this directory is for testing the GridTrust native device, which consists of an LM95172 temperature sensor connected to an Arduino Uno. 

The code will read from serial port `ttyACM0` and log temperatures (in Celsius) to the terminal every 3 seconds. This program does not have a stop condition.

Compilation and running:

`cargo run` in the same directory as `Cargo.toml`

Tested on Fedora 38.
