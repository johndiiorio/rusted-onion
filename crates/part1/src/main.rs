use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("part1.txt").unwrap();
    let text = decrypt(&contents).expect("Error decrypting ascii85 text");

    let bytes: Vec<u8> = text
        .bytes()
        .map(|mut x| {
            x = x ^ 0b01010101;
            x = x.rotate_right(1);
            return x;
        })
        .collect();

    let decrypted = String::from_utf8_lossy(bytes.as_slice());
    println!("{}", decrypted);
}
