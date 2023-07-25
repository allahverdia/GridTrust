extern crate telnet;

use telnet::Telnet;
use std::env;
use std::io;
use std::time::Duration;
use std::{thread, time};
use std::io::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::io::Read;



pub fn upload_program_ext(filepath: String, port_name: String, baud_rate: u32){

let mut port = Telnet::connect(("172.23.2.36", 23), 256)
	.expect("Couldn't connect to the relay...");

let mut f = std::fs::File::open(filepath).unwrap();

let mut contents = String::new();
f.read_to_string(&mut contents)
    .expect("something went wrong reading the file");
let mut strings=contents.split("\r\n");
for s in strings{
	println!("{}", s);
	let output =s.as_bytes();
	port.write(output).expect("Write failed!");
	let enter= "\r\n".as_bytes();
	port.write(enter).expect("Write failed!");
	let wait=time::Duration::from_millis(50);
	thread::sleep(wait);
    }
}	


	
