
/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */


//use std::io::{self};
use std::process::Command;
//use std::path::Path;
//use std::io::stdout;
//use std::str;
//use bstr::ByteSlice;


pub fn upload_program_ext(filepath: String, port_name: String, baud_rate: String){

    let filearg: String = "-Uflash:w:".to_owned() + &filepath;
    let port_name_full: String = "-P".to_owned() + &port_name;
    let baud_rate_full: String = "-b".to_owned() + &baud_rate;

   let _output = Command::new("avrdude").arg("-v").arg("-patmega328p").arg("-carduino").arg(&port_name_full).arg(&baud_rate_full).arg("-D").arg(&filearg).output().expect("Upload Failed");

}
