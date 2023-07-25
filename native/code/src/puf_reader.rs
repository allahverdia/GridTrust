/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */


use std::io::{self, Write};
use std::time::Duration;
use std::io::BufReader;
use std::io::BufRead; 


pub fn get_puf_out_ext(port_name: String, baud_rate: u32) -> String{

    write_serial_ext(&port_name, &baud_rate);
    let my_str: String = read_serial_ext(&port_name, &baud_rate);
    return my_str;

}


fn read_serial_ext(port_name: &str, baud_rate: &u32) -> String
{
//Reads the encrypted counter produced by the PUF
   let port = serialport::new(port_name, *baud_rate)
   .timeout(Duration::from_millis(3000))
   .open()
   .expect("Failed to open serial port");

    let mut reader = BufReader::new(port);
    let mut my_str = String::new();
    reader.read_line(&mut my_str).unwrap_or_default();

    return my_str;


}

fn write_serial_ext(port_name: &str, baud_rate: &u32)
{
//Writes a character over serial to the PUF chip, triggering the PUF chip to produce an encrypted counter
    let mut port = serialport::new(port_name, *baud_rate)
    .timeout(Duration::from_millis(10))
    .open();

    let output = "a".as_bytes();
	match port{
    	Ok(ref mut port) => {
		    match port.write(output) {
		        Ok(_) => {
            //print!("{:?}", output);
	            std::io::stdout().flush().unwrap();
		        }
		        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
		        Err(e) => eprintln!("{:?}", e),
		        }
		    }
    	Err(e) => {
        eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
        ::std::process::exit(1);
    	}	

}

}


