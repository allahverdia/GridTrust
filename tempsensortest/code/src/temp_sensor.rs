/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */


//use std::io::{self, Write};
use std::time::Duration;
//use std::process::Command;
//use std::env;
//use std::fs::File;
//use std::path::Path;
//use clap::{App, AppSettings, Arg};
use std::io::BufReader;
use std::io::BufRead;


pub fn read_serial_ext(port_name: &str, baud_rate: &u32) -> String
{
	let mut s = String::new();
    let serial_port = serialport::new(port_name, *baud_rate)
    .timeout(Duration::from_millis(3000))
    .open()
    .expect("Failed to open serial port");

    let mut reader = BufReader::new(serial_port);
    let mut my_str: Vec<u8> = vec![0;50];

    //reader.read_line(&mut my_str).unwrap_or_default();
    let eof = 0xa;
    match reader.read_until(eof, &mut my_str){
	Ok(_t) => s = String::from_utf8_lossy(&my_str).to_string(),
	//Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
	Err(e) => println!("{:?}", e),



	}
    
    return s;

}
