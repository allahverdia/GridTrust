mod puf_reader;

const PUF_PORT: &str = "/dev/ttyACM1";
const PUF_PORT_BAUD: u32 = 115200;

fn main() {
    let counter = read_puf_counter();
    println!("Encrypted PUF Counter: {}", counter);
}

fn read_puf_counter() -> String {
    // This function reads the encrypted counter from the PUF and returns it as a string
    let enc_ctr = puf_reader::get_puf_out_ext(PUF_PORT.to_string(), PUF_PORT_BAUD);
    enc_ctr.to_string()
}
