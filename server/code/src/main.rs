/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

mod constants;

use warp::Filter;
use std::io::prelude::*;
use std::fs::File;
use postgres::{ Client, Error, NoTls };
use aes::Aes128;
use aes::cipher::{ BlockEncrypt, KeyInit, generic_array::GenericArray };
use typenum::U16;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::path::Path;
use std::thread;
use std::time::Duration;
use constants::{
    DeviceType,
    SubstationDevice,
    Counter,
    Update,
    SensorReading,
    DbReadRequest,
    DbReadResponse,
    UpdateStatus,
};

lazy_static! {
    static ref LOCAL_UPDATE: Mutex<constants::UpdateType> = Mutex::new(constants::UpdateType::None);
}

lazy_static! {
    static ref DBREQUESTER: Mutex<DbReadRequest> = Mutex::new(DbReadRequest {
        uuid: "1".to_string(),
        request_flag: false,
        thread_id: 999,
    });
}

lazy_static! {
    static ref DBRESPONDER: Mutex<DbReadResponse> = Mutex::new(DbReadResponse {
        uuid: "1".to_string(),
        classification: "1".to_string(),
        key: "1".to_string(),
        counter: "1253523254".to_string(),
        done_flag: true,
        thread_id: 999,
    });
}

lazy_static! {
    static ref DBUPDATER: Mutex<DbReadResponse> = Mutex::new(DbReadResponse {
        uuid: "1".to_string(),
        classification: "1".to_string(),
        key: "1".to_string(),
        counter: "1253523254".to_string(),
        done_flag: true,
        thread_id: 999,
    });
}

fn main() {
    thread::sleep(Duration::from_millis(10000));
    retrieve_update_mode().unwrap();

    thread::spawn(move || {
        serving();
    });
    thread::spawn(move || {
        database_operator_pull();
    });
    thread::spawn(move || {
        database_operator_push();
    });

    loop {
        thread::sleep(Duration::from_millis(1000));
    }
}

fn database_operator_pull() {
    //producer of database responses in a producer/consumer ring
    loop {
        thread::sleep(Duration::from_millis(100));
        let mut device = SubstationDevice {
            uuid: "1".to_string(),
            classification: "1".to_string(),
            key: "1".to_string(),
            counter: "1".to_string(),
        };
        let mut get_data: bool = false;
        let mut thread_id: u32 = 999;
        {
            let mut newdbrequest = get_dbrequest();
            if newdbrequest.request_flag == true {
                newdbrequest.request_flag = false;
                get_data = true;
                thread_id = newdbrequest.thread_id;
                let newuuid = newdbrequest.uuid.clone();
                device = get_device(newuuid.clone()).unwrap();
                println!("uuid {}", newuuid);
                newdbrequest.uuid = "0".to_string();
                set_dbrequest(newdbrequest);
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }

        if get_data {
            let mut newdbresponse = get_dbresponse();
            if newdbresponse.done_flag == true {
                newdbresponse.done_flag = false;
                newdbresponse.uuid = device.uuid;
                newdbresponse.classification = device.classification;
                newdbresponse.key = device.key;
                newdbresponse.counter = device.counter;
                newdbresponse.thread_id = thread_id;
                set_dbresponse(newdbresponse);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn database_operator_push() {
    //consumer of database responses in a producer/consumer ring
    loop {
        thread::sleep(Duration::from_millis(100));
        {
            let mut newdbrequest = get_dbupdate();
            if newdbrequest.done_flag == false {
                newdbrequest.done_flag = true;
                update_device_ctr_auto(
                    newdbrequest.uuid.clone(),
                    newdbrequest.key.clone(),
                    newdbrequest.classification.clone(),
                    newdbrequest.counter.clone()
                ).unwrap();
                set_dbupdate(newdbrequest);
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn database_request_update(
    uuid: String,
    key: String,
    classification: String,
    counter: String
) -> Result<(), Error> {
    loop {
        thread::sleep(Duration::from_millis(100));
        {
            let mut newdbrequest = get_dbupdate();
            if newdbrequest.done_flag == true {
                newdbrequest.done_flag = false;
                newdbrequest.uuid = uuid;
                newdbrequest.key = key;
                newdbrequest.classification = classification;
                newdbrequest.counter = counter;
                set_dbupdate(newdbrequest);
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn get_device(uuid: String) -> Result<SubstationDevice, Error> {
    let mut device = SubstationDevice {
        uuid: "".to_string(),
        classification: "".to_string(),
        key: "".to_string(),
        counter: "".to_string(),
    };

    if uuid.clone().parse::<u128>().is_ok() {
        let mut client = Client::connect(
            "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
            NoTls
        )?;

        for row in client.query("SELECT * FROM substationDevices WHERE UUID=$1", &[&uuid])? {
            device.uuid = row.get(1);
            device.classification = row.get(2);
            device.key = row.get(3);
            device.counter = row.get(4);
        }
    } else {
        println!("Bad UUID");
    }

    Ok(device)
}

#[tokio::main]
async fn serving() {
    let testing = warp::path("test").map(|| { format!("Hello") });

    let tempread = warp
        ::path("tempread")
        .and(warp::body::json())
        .then(|sensor: SensorReading| async move {
            let update = Update {
                vendor_sig: "".to_string(),
                util_sig: "".to_string(),
                update_bin: "".to_string(),
                update_type: get_update_type(),
            };

            println!("{:?}", sensor.value);
            warp::reply::json(&update)
        });

    let updatestatus = warp
        ::path("updatestatus")
        .and(warp::body::json())
        .then(|updatestatus: UpdateStatus| async move {
            if updatestatus.status == true {
                println!(
                    "Device of type {:?} with UUID {} updated successfully",
                    updatestatus.device_type,
                    updatestatus.device_id
                );
            } else {
                println!(
                    "Device of type {:?} with UUID {} failed to update",
                    updatestatus.device_type,
                    updatestatus.device_id
                );
            }

            warp::reply::json(&updatestatus)
        });

    let pufread = warp
        ::post()
        .and(warp::path("pufread"))
        .and(warp::body::content_length_limit(1024 * 16))
        .and(warp::body::json())
        .then(move |counter: Counter| async {
            warp::reply::json(&serve_pufread(counter.enc_ctr, counter.uuid, counter.device_type))
        });

    let routes = tempread.or(pufread).or(testing).or(updatestatus);

    warp::serve(routes).run(([127, 0, 0, 1], 3031)).await;
}

fn serve_pufread(enc_ctr: String, uuid: String, device_type: DeviceType) -> Update {
    let mut thread_id: u32 = 0;
    loop {
        let mut device = SubstationDevice {
            uuid: "1".to_string(),
            classification: "1".to_string(),
            key: "1".to_string(),
            counter: "1".to_string(),
        };
        let mut get_data: bool = false;
        {
            thread::sleep(Duration::from_millis(100));
            let mut newdbrequest = get_dbrequest();

            if newdbrequest.request_flag == false {
                if uuid.clone().parse::<u128>().is_ok() {
                    thread_id = uuid.clone().parse().unwrap();
                    get_data = true;
                    newdbrequest.request_flag = true;
                    newdbrequest.uuid = uuid.clone();
                    newdbrequest.thread_id = thread_id.clone();
                    set_dbrequest(newdbrequest);
                } else {
                    println!("Bad UUID");
                    return Update {
                        vendor_sig: "".to_string(),
                        util_sig: "".to_string(),
                        update_bin: "".to_string(),
                        update_type: constants::UpdateType::None,
                    };
                }
            } else {
                thread::sleep(Duration::from_millis(100));
            }
        }

        if get_data {
            loop {
                thread::sleep(Duration::from_millis(100));
                let mut newdbresponse = get_dbresponse();
                println!("thread_id requesting {}", thread_id);
                println!("thread_id serving {}", newdbresponse.thread_id);
                if newdbresponse.done_flag == false && newdbresponse.thread_id == thread_id {
                    newdbresponse.done_flag = true;
                    device.uuid = newdbresponse.uuid.clone();
                    device.classification = newdbresponse.classification.clone();
                    device.key = newdbresponse.key.clone();
                    device.counter = newdbresponse.counter.clone();
                    println!("counter {}", device.counter);
                    set_dbresponse(newdbresponse);
                    return verify_device_ctr_ext(
                        enc_ctr,
                        uuid,
                        device.counter.clone(),
                        device.key.clone(),
                        device_type
                    );
                }
                thread::sleep(Duration::from_millis(100));
            }
        }
    }
}

fn verify_device_ctr_ext(
    enc_ctr: String,
    uuid: String,
    counter: String,
    key: String,
    device_type: DeviceType
) -> Update {
    println!("deserialized = {}", enc_ctr);
    let substrings = enc_ctr.split_whitespace();
    let mut ctr = String::new();
    for s in substrings {
        ctr += &s;
    }

    let device = retrieve_counter(counter, key, enc_ctr).unwrap();
    println!("device {}", device.counter);

    if device.counter != "" {
        let id = uuid;
        database_request_update(
            id.clone(),
            device.key,
            device.classification,
            device.counter
        ).unwrap();
        let mut update = Update {
            vendor_sig: "".to_string(),
            util_sig: "".to_string(),
            update_bin: "".to_string(),
            update_type: get_update_type(),
        };
        if get_update_type() == constants::UpdateType::Push {
            match device_type {
                DeviceType::TemperatureSensor => {
                    update = retrieve_update_files_temp();
                }
                DeviceType::RTU => {
                    update = retrieve_update_files_rtu();
                }
                DeviceType::Relay => {
                    update = retrieve_update_files_relay();
                }
            }
        }
        return update;
    }

    let update = Update {
        vendor_sig: "".to_string(),
        util_sig: "".to_string(),
        update_bin: "".to_string(),
        update_type: constants::UpdateType::None,
    };

    return update;
}

fn retrieve_update_files_temp() -> Update {
    let mut update = Update {
        vendor_sig: "".to_string(),
        util_sig: "".to_string(),
        update_bin: "".to_string(),
        update_type: get_update_type(),
    };

    if Path::new(&constants::VENDOR_SIG_FILE_PATH_TEMP).exists() {
        let mut file1 = File::open(&constants::VENDOR_SIG_FILE_PATH_TEMP).unwrap();
        let mut vendor_sig = String::new();
        file1.read_to_string(&mut vendor_sig).unwrap();
        update.vendor_sig = vendor_sig;
    }

    if Path::new(&constants::UPDATE_FILE_PATH_TEMP).exists() {
        let mut file2 = File::open(&constants::UPDATE_FILE_PATH_TEMP).unwrap();
        let mut update_bin = String::new();
        file2.read_to_string(&mut update_bin).unwrap();
        update.update_bin = update_bin;
    }

    if Path::new(&constants::UTIL_SIG_FILE_PATH_TEMP).exists() {
        let mut file3 = File::open(&constants::UTIL_SIG_FILE_PATH_TEMP).unwrap();
        let mut util_sig = String::new();
        file3.read_to_string(&mut util_sig).unwrap();
        update.util_sig = util_sig;
    }
    update
}

fn retrieve_update_files_relay() -> Update {
    let mut update = Update {
        vendor_sig: "".to_string(),
        util_sig: "".to_string(),
        update_bin: "".to_string(),
        update_type: get_update_type(),
    };

    if Path::new(&constants::VENDOR_SIG_FILE_PATH_RELAY).exists() {
        let mut file1 = File::open(&constants::VENDOR_SIG_FILE_PATH_RELAY).unwrap();
        let mut vendor_sig = String::new();
        file1.read_to_string(&mut vendor_sig).unwrap();
        update.vendor_sig = vendor_sig;
    }

    if Path::new(&constants::UPDATE_FILE_PATH_RELAY).exists() {
        let mut file2 = File::open(&constants::UPDATE_FILE_PATH_RELAY).unwrap();
        let mut update_bin = String::new();
        file2.read_to_string(&mut update_bin).unwrap();
        update.update_bin = update_bin;
    }

    if Path::new(&constants::UTIL_SIG_FILE_PATH_RELAY).exists() {
        let mut file3 = File::open(&constants::UTIL_SIG_FILE_PATH_RELAY).unwrap();
        let mut util_sig = String::new();
        file3.read_to_string(&mut util_sig).unwrap();
        update.util_sig = util_sig;
    }
    update
}

fn retrieve_update_files_rtu() -> Update {
    let mut update = Update {
        vendor_sig: "".to_string(),
        util_sig: "".to_string(),
        update_bin: "".to_string(),
        update_type: get_update_type(),
    };

    if Path::new(&constants::VENDOR_SIG_FILE_PATH_RTU).exists() {
        let mut file1 = File::open(&constants::VENDOR_SIG_FILE_PATH_RTU).unwrap();
        let mut vendor_sig = String::new();
        file1.read_to_string(&mut vendor_sig).unwrap();
        update.vendor_sig = vendor_sig;
    }

    if Path::new(&constants::UPDATE_FILE_PATH_RTU).exists() {
        let mut file2 = File::open(&constants::UPDATE_FILE_PATH_RTU).unwrap();
        let mut update_bin = String::new();
        file2.read_to_string(&mut update_bin).unwrap();
        update.update_bin = update_bin;
    }

    if Path::new(&constants::UTIL_SIG_FILE_PATH_RTU).exists() {
        let mut file3 = File::open(&constants::UTIL_SIG_FILE_PATH_RTU).unwrap();
        let mut util_sig = String::new();
        file3.read_to_string(&mut util_sig).unwrap();
        update.util_sig = util_sig;
    }
    update
}

fn increment_counter(ctr: String) -> String {
    println!("initial counter {}", ctr);
    let number = parse_hex(&ctr);
    let mut number =
        ((number[0] as u128) << 120) |
        ((number[1] as u128) << 112) |
        ((number[2] as u128) << 104) |
        ((number[3] as u128) << 96) |
        ((number[4] as u128) << 88) |
        ((number[5] as u128) << 80) |
        ((number[6] as u128) << 72) |
        ((number[7] as u128) << 64) |
        ((number[8] as u128) << 56) |
        ((number[9] as u128) << 48) |
        ((number[10] as u128) << 40) |
        ((number[11] as u128) << 32) |
        ((number[12] as u128) << 24) |
        ((number[13] as u128) << 16) |
        ((number[14] as u128) << 8) |
        (number[15] as u128);

    number = number + 1;

    let number = format!("{:032x}", number);
    println!("inc counter {}", number);
    return number;
}

fn retrieve_counter(
    counter: String,
    key: String,
    enc_ctr: String
) -> Result<SubstationDevice, Error> {
    let enc_ctr = parse_hex(&enc_ctr);

    let mut device = SubstationDevice {
        uuid: "".to_string(),
        classification: "".to_string(),
        key: key,
        counter: counter,
    };

    let empty_device = SubstationDevice {
        uuid: "".to_string(),
        classification: "".to_string(),
        key: "".to_string(),
        counter: "".to_string(),
    };

    println!("Database stored values:\r\nKey: {:?}, Counter: {:?}", device.key, device.counter);

    for _n in 0..constants::NUM_ENC_TRIES {
        let block = parse_hex(&device.counter);
        let newkey = parse_hex(&device.key);
        println!("Counter {:x?}", &device.counter);
        println!("Key {:x?}", &device.key);
        let ciphertext = return_ciphertext(&newkey, &block);

        println!("enc_ctr from Server: {:x?}", &ciphertext);
        println!("enc_ctr from Device: {:x?}", &enc_ctr);

        if ciphertext == enc_ctr {
            println!("Ctrs Match");
            device.counter = increment_counter(device.counter);
            return Ok(device);
        } else {
            println!("Checking for ctr desync");
            device.counter = increment_counter(device.counter);
        }
    }

    return Ok(empty_device);
}

fn return_ciphertext(key: &[u8], block: &[u8]) -> Vec<u8> {
    let keybytes: GenericArray<_, U16> = GenericArray::clone_from_slice(&key[0..16]);
    let mut newblock: GenericArray<_, U16> = GenericArray::clone_from_slice(&block[0..16]);
    let cipher = Aes128::new(&keybytes);
    cipher.encrypt_block(&mut newblock);
    let newblock = newblock.to_vec();
    return newblock;
}

fn parse_hex(hex_asm: &str) -> Vec<u8> {
    //Function to convert a string of hex values into the actual bit-wise representation (i.e., the string "123abc" -> 0x123abc)
    let mut hex_bytes = hex_asm
        .as_bytes()
        .iter()
        .filter_map(|b| {
            match b {
                b'0'..=b'9' => Some(b - b'0'),
                b'a'..=b'f' => Some(b - b'a' + 10),
                b'A'..=b'F' => Some(b - b'A' + 10),
                _ => None,
            }
        })
        .fuse();

    let mut bytes = Vec::new();
    while let (Some(h), Some(l)) = (hex_bytes.next(), hex_bytes.next()) {
        bytes.push((h << 4) | l);
    }
    bytes
}

fn update_device_ctr_auto(
    uuid: String,
    key: String,
    classification: String,
    counter: String
) -> Result<(), Error> {
    println!("Updating Device");
    println!("uuid {}", uuid);

    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls
    )?;

    client.execute(
        "UPDATE substationDevices SET classification=$1, key=$2, counter=$3  WHERE UUID=$4",
        &[&classification, &key, &counter, &uuid]
    )?;
    Ok(())
}

fn retrieve_update_mode() -> Result<(), Error> {
    let mut s = "".to_string();
    let mut client = Client::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/postgres",
        NoTls
    )?;
    for row in client.query("SELECT * FROM updatetype", &[])? {
        s = row.get(0);
    }
    if s == "Push".to_string() {
        set_update_type(constants::UpdateType::Push);
    } else if s == "Local".to_string() {
        set_update_type(constants::UpdateType::Local);
    }
    println!("{:?}", s);
    Ok(())
}

fn set_dbresponse(dbresponse: DbReadResponse) {
    *DBRESPONDER.lock().unwrap() = dbresponse;
}

fn get_dbresponse() -> DbReadResponse {
    DBRESPONDER.lock().unwrap().clone()
}

fn set_dbrequest(dbrequest: DbReadRequest) {
    *DBREQUESTER.lock().unwrap() = dbrequest;
}

fn get_dbrequest() -> DbReadRequest {
    DBREQUESTER.lock().unwrap().clone()
}

fn set_dbupdate(dbupdate: DbReadResponse) {
    *DBUPDATER.lock().unwrap() = dbupdate;
}

fn get_dbupdate() -> DbReadResponse {
    DBUPDATER.lock().unwrap().clone()
}

fn get_update_type() -> constants::UpdateType {
    LOCAL_UPDATE.lock().unwrap().clone()
}

fn set_update_type(update_type: constants::UpdateType) {
    *LOCAL_UPDATE.lock().unwrap() = update_type;
}
