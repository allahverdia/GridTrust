/* Copyright (c) 2023 Kevin Hutto

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT,TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. */

mod puf_reader;
mod updater;
mod relay;
use std::fs::File;
use std::io::{ Write };
use serde::{ Serialize, Deserialize };
use std::process::Command;
use std::io::Read;
use openssl::sign::Verifier;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use std::vec::Vec;
use subprocess::{ PopenError };
use std::time::Duration;
use std::thread;
use std::path::Path;
//These constants are the only change for each device. The code should have no other change for deploying on different devices.
const DEVICETYPE: DeviceType = DeviceType::Relay; //ADJUST FOR EACH DEVICE. Enums are defined below

const PUF_PORT: &str = "/dev/ttyACM0"; //ADJUST FOR EACH DEVICE. ACM is for arduinos/PUfs, USB for relay. The number is assigned based on which device is plugged in first
const PUF_PORT_BAUD: u32 = 115200; //ADJUST FOR EACH DEVICE

const POWER_DEVICE_PORT: &str = "/dev/ttyUSB0"; //ADJUST FOR EACH DEVICE
const POWER_DEVICE_PORT_BAUD: u32 = 19200; //9600;                                //ADJUST FOR EACH DEVICE
//const POWER_DEVICE_BAUD_UPDATE:&str = "115200";                         //ADJUST FOR EACH DEVICE. Arduino uses a different baud for uploading a new program than it does for communication

//device ID used for server database management
const UUID: &str = "2"; //ADJUST FOR EACH DEVICE. The current database on the server assume tempsensor = 1, relay = 2, rtu = 3

//Files created when unpacking JSON from server
const UTIL_SIG_FILE_PATH: &str = "./updater/files_to_use/util_sig.txt";
const VENDOR_SIG_FILE_PATH: &str = "./updater/files_to_use/vendor_sig.txt";
const UPDATE_FILE_PATH: &str = "./updater/files_to_use/update64.txt"; //ADJUST FOR EACH DEVICE
const UPDATE_DIR: &str = "./updater/files_to_use";

const CERT_FILE_PATH: &str = "./updater/gridtrust.pfx"; //ADJUST FOR EACH DEVICE
const CERT_FILE_PATH_PEM: &str = "./updater/ca2.crt";

const LOCAL_FIL_DIR: &str = "./local_updates/";
const LOCAL_UTIL_SIG_FILE_PATH: &str = "util_sign64.txt";
const LOCAL_VENDOR_SIG_FILE_PATH: &str = "vendor_sign64.txt";
const LOCAL_UPDATE_FILE_PATH: &str = "update64.txt"; //ADJUST FOR EACH DEVICE
//Files created when converting transmitted signatures into
//OPENSSL signature format
const UTIL_SIG_FILE_PATH_CONV: &str = "./updater/files_to_use/util_sig_conv.txt";
const VENDOR_SIG_FILE_PATH_CONV: &str = "./updater/files_to_use/vendor_sig_conv.txt";
const UPDATE_FILE_PATH_CONV: &str = "./updater/files_to_use/update.txt";

const UTIL_PUB_KEY: &str = "./updater/utility.pub.pem";
const VENDOR_PUB_KEY: &str = "./updater/vendor.pub.pem";

//Enum for what type of device the client is
#[derive(Deserialize, Serialize)]
enum DeviceType {
    RTU,
    TemperatureSensor,
    Relay,
}

//Enums for the server communicating what update type to use
#[derive(Serialize, Deserialize, PartialEq, Eq, Clone)]
pub enum UpdateType {
    None,
    Local,
    Push,
}

//http post for PUF counter
#[derive(Serialize, Deserialize)]
struct Counter {
    uuid: String,
    enc_ctr: String,
    device_type: DeviceType,
}

//http get for update
#[derive(Serialize, Deserialize)]
struct Update {
    vendor_sig: String,
    util_sig: String,
    update_bin: String,
    update_type: UpdateType,
}

//http post for temp reading
#[derive(Serialize, Deserialize, Debug)]
struct SensorReading {
    value: String,
}

//http post for update status
#[derive(Serialize, Deserialize)]
struct UpdateStatus {
    device_id: String,
    device_type: DeviceType,
    status: bool,
}

fn main() {
    match DEVICETYPE {
        DeviceType::TemperatureSensor => main_relay(),
        DeviceType::RTU => main_relay(),
        DeviceType::Relay => main_relay(),
    }
}

fn main_relay() {
    //Same as main_temp but delays some amount of time instead of read temperatures

    loop {
        thread::sleep(Duration::from_millis(3000));
        let counter = push_puf_authentication();

        let update_package: Update = callserverpuf(counter).unwrap();
        if check_for_update(update_package).unwrap_or(false) {
            let mut updatestatus = UpdateStatus {
                device_id: UUID.to_string(),
                device_type: DEVICETYPE,
                status: false,
            };
            convert_signatures().unwrap();
            if check_signatures() {
                push_updates_relay(); //Unique update function
                updatestatus.status = true;
            }
            callserverupdatestatus(updatestatus).unwrap();
            clean_files().unwrap();
        }
        thread::sleep(Duration::from_millis(30000));
    }
}

//function posts the puf reading to the server, gets an update response
#[tokio::main]
async fn callserverpuf(counter: Counter) -> Result<Update, reqwest::Error> {
    let mut buf = Vec::new();
    File::open(&CERT_FILE_PATH.to_owned()).unwrap().read_to_end(&mut buf).unwrap();
    let id = reqwest::Identity::from_pkcs12_der(&buf, "testpassword")?; //"testpassword" here is the password for the .pfx certificate. If you change the certificate and the password, this must be changed.

    let mut buf2 = Vec::new();
    File::open(&CERT_FILE_PATH_PEM.to_owned()).unwrap().read_to_end(&mut buf2).unwrap();
    let cert = reqwest::Certificate::from_pem(&buf2)?;

    let client = reqwest::Client
        ::builder()
        .add_root_certificate(cert)
        .tls_built_in_root_certs(false)
        .danger_accept_invalid_hostnames(true)
        .identity(id)
        .build()?;

    let new_post: Update = client
        .post("https://172.23.2.200:3030/pufread") //CHANGE HTTPS TO HTTP FOR NON TLS
        .json(&counter)
        .send().await?
        .json().await?;
    Ok(new_post)
}

#[tokio::main]
async fn callserverupdatestatus(updatestatus: UpdateStatus) -> Result<(), reqwest::Error> {
    let mut buf = Vec::new();
    File::open(&CERT_FILE_PATH.to_owned()).unwrap().read_to_end(&mut buf).unwrap();
    let id = reqwest::Identity::from_pkcs12_der(&buf, "test%^&098")?;

    let mut buf2 = Vec::new();
    File::open(&CERT_FILE_PATH_PEM.to_owned()).unwrap().read_to_end(&mut buf2).unwrap();
    let cert = reqwest::Certificate::from_pem(&buf2)?;

    let client = reqwest::Client
        ::builder()
        .add_root_certificate(cert)
        .tls_built_in_root_certs(false)
        .danger_accept_invalid_hostnames(true)
        .identity(id)
        .build()?;
    client
        .post("https://172.23.2.200:3030/updatestatus") //CHANGE HTTPS TO HTTP FOR NON TLS
        .json(&updatestatus)
        .send().await?;
    Ok(())
}

fn push_puf_authentication() -> Counter {
    //This function gets the current encryted counter from the PUF and packages the value into a struct

    let enc_ctr = puf_reader::get_puf_out_ext(PUF_PORT.to_string(), PUF_PORT_BAUD);
    println!("Enc Counter at client: {}", enc_ctr);

    let data = Counter {
        uuid: UUID.to_string(),
        enc_ctr: enc_ctr.to_string(),
        device_type: DEVICETYPE,
    };

    data
}

fn check_for_update(deserialized: Update) -> std::io::Result<bool> {
    //This function returns true if the provided Update is non empty
    //The server provides a non-empty Update when there is an update

    if deserialized.update_type == UpdateType::Push {
        println!("Checking for pushed update files");
        let vendor_sig: String = deserialized.vendor_sig;
        let util_sig: String = deserialized.util_sig;
        let update_bin: String = deserialized.update_bin;

        if vendor_sig != "" && util_sig != "" && update_bin != "" {
            println!("{}", vendor_sig);
            println!("{}", util_sig);
            println!("{}", update_bin);
            println!("update found");
            std::fs::create_dir_all("/updater/files_to_use".to_string())?;
            let mut file = File::create(&VENDOR_SIG_FILE_PATH)?;
            write!(file, "{}", vendor_sig)?;
            let mut file = File::create(&UTIL_SIG_FILE_PATH)?;
            write!(file, "{}", util_sig)?;
            let mut file = File::create(&UPDATE_FILE_PATH)?;
            write!(file, "{}", update_bin)?;
            Ok(true)
        } else {
            println!("No Update Pushed, Expected Update Pushed");
            Ok(false)
        }
    } else if deserialized.update_type == UpdateType::Local {
        println!("Installing Update from local files");

        //Open files in local directory
        println!("directory {}", LOCAL_FIL_DIR.to_owned() + &LOCAL_UPDATE_FILE_PATH.to_owned());
        if
            Path::new(&(LOCAL_FIL_DIR.to_owned() + &LOCAL_UPDATE_FILE_PATH.to_owned())).exists() &&
            Path::new(
                &(LOCAL_FIL_DIR.to_owned() + &LOCAL_UTIL_SIG_FILE_PATH.to_owned())
            ).exists() &&
            Path::new(&(LOCAL_FIL_DIR.to_owned() + &LOCAL_VENDOR_SIG_FILE_PATH.to_owned())).exists()
        {
            std::fs::create_dir_all(&UPDATE_DIR)?;
            //Create a copy of the provided vendor signature
            let mut f = std::fs::File
                ::open(&(LOCAL_FIL_DIR.to_owned() + &LOCAL_VENDOR_SIG_FILE_PATH.to_owned()))
                .unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect("something went wrong reading vendor signature");
            let mut file = File::create(&VENDOR_SIG_FILE_PATH)?;
            write!(file, "{}", contents)?;
            //Create a copy of the provided utility signature
            let mut f = std::fs::File
                ::open(&(LOCAL_FIL_DIR.to_owned() + &LOCAL_UTIL_SIG_FILE_PATH.to_owned()))
                .unwrap();
            let mut contents = String::new();
            f.read_to_string(&mut contents).expect(
                "something went wrong reading utility signature"
            );
            let mut file = File::create(&UTIL_SIG_FILE_PATH)?;
            write!(file, "{}", contents)?;
            //Create a copy of the provided update binary
            let mut f = std::fs::File
                ::open(&(LOCAL_FIL_DIR.to_owned() + &LOCAL_UPDATE_FILE_PATH.to_owned()))
                .unwrap();
            let mut contents = Vec::new();
            f.read_to_end(&mut contents).expect("something went wrong reading update file");
            let mut file = File::create(&UPDATE_FILE_PATH)?;
            let contents = std::str::from_utf8(&contents).unwrap().to_string();
            write!(file, "{}", contents)?;
            Ok(true)
        } else {
            println!("No local update files");
            Ok(false)
        }
    } else {
        println!("No update scheduled");
        Ok(false)
    }
}

fn convert_signatures() -> Result<(), PopenError> {
    println!("checking update signatures");

    Command::new("sh").arg("./base64.sh").output().expect("Signature Conversion Failure");

    Ok(())
}

fn check_signatures() -> bool {
    //loads both signatures, public keys, update file, and verifies the signatures are valid.
    //Intended to be reworked to take in file paths as args, calling this function twice instead of once.

    let mut vendor_pub_file = File::open(&VENDOR_PUB_KEY).unwrap();
    let mut vendor_pem = String::new();
    vendor_pub_file.read_to_string(&mut vendor_pem).unwrap();
    let vendor_key = PKey::public_key_from_pem(vendor_pem.as_bytes()).unwrap();
    let mut vendor = Verifier::new(MessageDigest::sha3_256(), &vendor_key).unwrap();
    let mut check_file = File::open(&UPDATE_FILE_PATH_CONV).unwrap();
    let mut vendor_sig_file = File::open(&VENDOR_SIG_FILE_PATH_CONV).unwrap();
    let mut unknown = String::new();
    let mut vendor_sig = Vec::new();
    check_file.read_to_string(&mut unknown).unwrap();
    vendor_sig_file.read_to_end(&mut vendor_sig).unwrap();
    vendor.update(unknown.as_bytes()).unwrap();

    let mut util_pub_file = File::open(&UTIL_PUB_KEY).unwrap();
    let mut util_pem = String::new();
    util_pub_file.read_to_string(&mut util_pem).unwrap();
    let util_key = PKey::public_key_from_pem(util_pem.as_bytes()).unwrap();
    let mut util = Verifier::new(MessageDigest::sha3_256(), &util_key).unwrap();
    let mut check_file = File::open(&UPDATE_FILE_PATH_CONV).unwrap();
    let mut util_sig_file = File::open(&UTIL_SIG_FILE_PATH_CONV).unwrap();
    let mut unknown = String::new();
    let mut util_sig = Vec::new();
    check_file.read_to_string(&mut unknown).unwrap();
    util_sig_file.read_to_end(&mut util_sig).unwrap();
    util.update(unknown.as_bytes()).unwrap();

    if util.verify(&util_sig).unwrap() == false {
        println!("utility failed");
    }
    if vendor.verify(&vendor_sig).unwrap() == false {
        println!("vendor failed");
    }

    if util.verify(&util_sig).unwrap() && vendor.verify(&vendor_sig).unwrap() {
        return true;
    } else {
        println!("update rejected");
        return false;
    }
}

fn push_updates_relay() {
    //Calls the relay class function to upload the new binary file
    println!("installing update");
    relay::upload_program_ext(
        UPDATE_FILE_PATH_CONV.to_string(),
        POWER_DEVICE_PORT.to_string(),
        POWER_DEVICE_PORT_BAUD
    );
}

fn clean_files() -> std::io::Result<()> { // Ensure that this function *does not* delete the ./updater/files_to_use directory, but clears its contents
    println!("Removing Intermediate Update Files");
    for entry in std::fs::read_dir(&UPDATE_DIR)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            std::fs::remove_file(path)?;
        }
        else if path.is_dir() {
            std::fs::remove_dir_all(path)?;
        }
    }
    Ok(())
}
