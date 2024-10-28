mod relay;
use std::fs::File;
use std::io::{ Write };
use std::process::Command;
use std::io::Read;
use std::vec::Vec;
use subprocess::{ PopenError };
use std::time::Duration;
use std::thread;
use std::path::Path;

const UPDATE_FILE_PATH: &str = "./update.txt";
const POWER_DEVICE_PORT: &str = "/dev/ttyUSB0";
const POWER_DEVICE_PORT_BAUD: u32 = 19200;

fn main() {
    push_updates_relay();
}

fn push_updates_relay() {
    //Calls the relay class function to upload the update file
    println!("installing update");
    relay::upload_program_ext(
        UPDATE_FILE_PATH.to_string(),
        POWER_DEVICE_PORT.to_string(),
        POWER_DEVICE_PORT_BAUD
    );
}
