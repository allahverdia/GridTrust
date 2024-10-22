/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

use serde::{ Serialize, Deserialize };

pub const UTIL_SIG_FILE_PATH_TEMP: &str = "./update_files/tempsensor/util_sign64.txt";
pub const VENDOR_SIG_FILE_PATH_TEMP: &str = "./update_files/tempsensor/vendor_sign64.txt";
pub const UPDATE_FILE_PATH_TEMP: &str = "./update_files/tempsensor/update64.txt";

pub const UTIL_SIG_FILE_PATH_RTU: &str = "./update_files/rtu/util_sign64.txt";
pub const VENDOR_SIG_FILE_PATH_RTU: &str = "./update_files/rtu/vendor_sign64.txt";
pub const UPDATE_FILE_PATH_RTU: &str = "./update_files/rtu/update64.txt";

pub const UTIL_SIG_FILE_PATH_RELAY: &str = "./update_files/relay/util_sign64.txt";
pub const VENDOR_SIG_FILE_PATH_RELAY: &str = "./update_files/relay/vendor_sign64.txt";
pub const UPDATE_FILE_PATH_RELAY: &str = "./update_files/relay/update64.txt";

pub const NUM_ENC_TRIES: u32 = 5;

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceType {
    RTU,
    TemperatureSensor,
    Relay,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum UpdateType {
    None,
    Local,
    Push,
}

#[derive(Clone)]
pub struct SubstationDevice {
    pub uuid: String,
    pub classification: String,
    pub key: String,
    pub counter: String,
}

#[derive(Serialize, Deserialize)]
pub struct Counter {
    pub uuid: String,
    pub enc_ctr: String,
    pub device_type: DeviceType,
}

#[derive(Serialize, Deserialize)]
pub struct Update {
    pub vendor_sig: String,
    pub util_sig: String,
    pub update_bin: String,
    pub update_type: UpdateType,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateStatus {
    pub device_id: String,
    pub device_type: DeviceType,
    pub status: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorReading {
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct DbReadRequest {
    pub uuid: String,
    pub request_flag: bool,
    pub thread_id: u32,
}

#[derive(Clone)]
pub struct DbReadResponse {
    pub uuid: String,
    pub classification: String,
    pub key: String,
    pub counter: String,
    pub done_flag: bool,
    pub thread_id: u32,
}
