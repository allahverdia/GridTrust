The code in this directory is for testing the PUF board. The code will read from serial port `ttyACM1` and log the encrypted (with AES-128 ctr mode) counter to the terminal. 

By default (when the board has been enrolled and subsequently programmed with the encrypt counter project), the AES key on the PUF board is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`

Initially, the counter that AES uses for encryption is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00`

 The plaintext that is encrypted is always `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`

The first time you run the program, you should see an output of `05 45 aa d5 6d a2 a9 7c 36 63 d1 43 2a 3d 1c 85`

Each time the program runs, the encrypted counter will be different because the counter on the board is being incremented with each run. 

For example, the second time you run the program, the board will encrypt `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` using counter `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` and so on. The encryption key never changes.

Compilation and running:

`cargo run` in the same directory as `Cargo.toml`
