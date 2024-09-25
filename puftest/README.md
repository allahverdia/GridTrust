The code in this directory is for testing the PUF board. The code will read from serial port `ttyACM1` and log the encrypted (with AES-128 ctr mode) counter to the terminal. 

By default, the AES key on the PUF board is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`

Note that each time the program runs, the encrypted counter will be different because the counter on the board is being incremented with each run.

Compilation and running:

`cargo run` in the same directory as `Cargo.toml`
