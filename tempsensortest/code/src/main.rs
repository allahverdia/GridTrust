mod temp_sensor;

const POWER_DEVICE_PORT:&str = "/dev/ttyACM0"; 
const POWER_DEVICE_PORT_BAUD:u32 = 9600;    
fn main() {
    loop {
        transmit_temp();
    }
}

fn transmit_temp() {

    let temperature = temp_sensor::read_serial_ext(&POWER_DEVICE_PORT.to_string(), &POWER_DEVICE_PORT_BAUD);


    let mut substrings = temperature.split_whitespace();
    substrings.next();
    let temperature = substrings.next();
    
    let opt: Option<&str> = temperature;
    let value: Option<String> = opt.map(str::to_string);
    let temperature = value.unwrap_or_default();

    // Print the temperature to the console
    println!("{}", temperature);
}
