use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("payloads/part3.txt").unwrap();
    let bytes = decrypt(&contents).expect("Error decrypting ascii85 text");
    let mut key: Vec<u8> = vec![0; 32];

    // To get the plaintext header, first guess that the first part of
    // the header is "==[ Layer 4/5 " and the last part is "=====", which is
    // based on the header of the previous parts. This leads to text that is close enough
    // to infer that the title of the section next section is "Network Traffic".
    let plaintext_header = "==[ Layer 4/5: Network Traffic ]";
    for (i, c) in plaintext_header.chars().enumerate() {
        key[i] = (c as u8) ^ bytes[i];
    }

    let final_bytes: Vec<u8> = bytes
        .iter()
        .enumerate()
        .map(|(i, byte)| {
            return byte ^ key[i % 32];
        })
        .collect();

    let decrypted = String::from_utf8(final_bytes).unwrap();
    println!("{}", decrypted);
}
