use ascii85::decrypt;
use std::fs::read_to_string;

mod packet;

use crate::packet::Packet;

fn main() {
    let contents = read_to_string("part4.txt").unwrap();
    let bytes = decrypt(&contents).expect("Error decrypting ascii85 text");

    let packet = Packet::new(0, &bytes);

    println!("{}", packet.is_valid());
    let _packet_data = packet.get_data();
}
