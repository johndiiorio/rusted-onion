use ascii85::decrypt;
use std::fs::read_to_string;

mod packet;

use crate::packet::Packet;

fn main() {
    let contents = read_to_string("payloads/part4.txt").unwrap();
    let bytes = decrypt(&contents).expect("Error decrypting ascii85 text");

    let mut data = Vec::new();
    let mut index: usize = 0;
    while index < bytes.len() {
        let packet = match Packet::new(index, &bytes) {
            Ok(p) => p,
            Err(_) => {
                break;
            }
        };
        if packet.is_valid() {
            let mut packet_data = packet.get_data().to_vec();
            data.append(&mut packet_data);
        }
        index += packet.get_packet_length() as usize;
    }

    let decrypted = String::from_utf8(data).unwrap();
    println!("{}", decrypted);
}
