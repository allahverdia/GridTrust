The code in this directory is for testing the PUF board. The program will read from serial port `ttyACM1` and log the encrypted (with AES-128 ECB mode) counter to the terminal. The term "counter" here does not imply that AES counter mode is being used for encryption. The "counter" is the plaintext value that gets encrypted.

This program triggers the PUF chip to produce an encrypted counter by writing the character `a` to the PUF board.

By default, the AES key on the PUF board is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`

Initially, the counter (plaintext) that AES encrypts is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00`

The first time you run the program, you should see an output of `05 45 aa d5 6d a2 a9 7c 36 63 d1 43 2a 3d 1c 84`

Each time the program runs, the encrypted value shown in the terminal will be different because the counter on the board is being incremented with each run. 

For example, the second time you run the program, the board will encrypt `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01` and so on. The encryption key never changes.

Compilation and running:

Ensure that the PUF board is connected to the computer via USB connection from the computer to the PUF's debug port

`cargo run` in the same directory as `Cargo.toml`
