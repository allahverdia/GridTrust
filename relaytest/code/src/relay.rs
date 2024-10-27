/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

//extern crate telnet;

use telnet::Telnet;
use std::time::Duration;
use std::{ thread, time };
use std::io::prelude::*;
use std::io::Read;

const DELAYWAIT1: u64 = 30000; //30000 for 351
const DELAYWAIT2: u64 = 7000; //5000 for 351

pub fn upload_program_ext(filepath: String, port_name: String, baud_rate: u32) {
    let mut port = serialport
        ::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open()
        .expect("Failed to open port");

    let mut f = std::fs::File::open(filepath).unwrap();

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("something went wrong reading the file");
    let strings = contents.split("\r\n");
    let mut i = 0;
    for s in strings {
        i += 1;
        if i == 101 {
            thread::sleep(Duration::from_millis(DELAYWAIT1));
        } else if i > 101 {
            thread::sleep(Duration::from_millis(DELAYWAIT2));
        }
        println!("{}", s);
        let output = s.as_bytes();
        port.write(output).expect("Write failed!");
        let enter = "\r\n".as_bytes();
        port.write(enter).expect("Write failed!");
        let wait = time::Duration::from_millis(50);
        thread::sleep(wait);
    }
}
