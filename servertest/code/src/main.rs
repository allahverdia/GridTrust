use aes::Aes128;
use aes::cipher::{BlockEncrypt, KeyInit, generic_array::GenericArray};
use postgres::{Client, NoTls, Error};
use typenum::U16;
use std::str::FromStr;

fn encrypt_counter(counter: &str, key: &str) -> Vec<u8> {
    let key_bytes: GenericArray<_, U16> = GenericArray::clone_from_slice(&hex_to_bytes(key));
    let mut counter_bytes: GenericArray<_, U16> = GenericArray::clone_from_slice(&hex_to_bytes(counter));
    
    let cipher = Aes128::new(&key_bytes);
    
    cipher.encrypt_block(&mut counter_bytes);
    
    counter_bytes.to_vec()
}

fn hex_to_bytes(hex: &str) -> Vec<u8> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
        .collect()
}

fn increment_counter(counter: &str) -> String {
    let counter_num = u128::from_str_radix(counter, 16).unwrap();
    format!("{:032x}", counter_num + 1)
}

fn read_encrypt_increment_counter(uuid: &str) -> Result<(), Error> {
    let mut client = Client::connect("postgresql://dboperator:operatorpass123@localhost:5243/postgres", NoTls)?;

    let mut counter = String::new();
    let mut key = String::new();
    
    for row in client.query("SELECT counter, key FROM substationDevices WHERE UUID=$1", &[&uuid])? {
        counter = row.get(0);
        key = row.get(1);
    }

    if !counter.is_empty() && !key.is_empty() {
        println!("Original Counter: {}", counter);

        let encrypted_counter = encrypt_counter(&counter, &key);
        // Print the encrypted counter in hex format
        println!("Encrypted Counter: {:x?}", encrypted_counter);

        let new_counter = increment_counter(&counter);
        println!("Incremented Counter: {}", new_counter);

        client.execute(
            "UPDATE substationDevices SET counter=$1 WHERE UUID=$2",
            &[&new_counter, &uuid],
        )?;
    } else {
        println!("Counter or Key not found in the database for UUID: {}", uuid);
    }

    Ok(())
}

fn main() {
    let uuid = "1"; // Change this based on the device whose key and counter you want to fetch from the database
    match read_encrypt_increment_counter(uuid) {
        Ok(_) => println!("Counter processed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
