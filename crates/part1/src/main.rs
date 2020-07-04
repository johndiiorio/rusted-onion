use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("part1.txt").unwrap();
    let text = decrypt(&contents).expect("Error decrypting ascii85 text");

    let modified_bytes: Vec<u8> = text
        .iter()
        .map(|x| {
            return (x ^ 0b01010101).rotate_right(1);
        })
        .collect();

    let decrypted = String::from_utf8(modified_bytes).unwrap();
    println!("{}", decrypted);
}
