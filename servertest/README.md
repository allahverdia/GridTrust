The code in this directory is for testing the communication between the server and the PostgreSQL database that is also running on the server machine. 

The program will read the key and counter value from the PostgreSQL database for the specified device UUID, and encrypt the counter using the key (with AES128-ECB).

Subsequently, the program will increment the counter and update the database with the incremented counter.

By default, the AES key in the database is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01`

Initially, the counter (plaintext) that AES encrypts is `00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00`

The first time you run the program, you should see an output of:
`Encrypted Counter: [5, 45, aa, d5, 6d, a2, a9, 7c, 36, 63, d1, 43, 2a, 3d, 1c, 84]`
`Counter: 00000000000000000000000000000001`
`Counter processed successfully`


Each time the program runs, the encrypted value shown in the terminal will be different because the counter in the database is being incremented with each run. 

Compilation and running:

Ensure Docker is running and the PostgreSQL database has been configured correctly according to https://mooney.gatech.edu/security/gridtrust/server/

`cargo run` in the same directory as `Cargo.toml`
