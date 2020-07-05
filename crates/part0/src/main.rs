use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("part0.txt").unwrap();
    let bytes = decrypt(&contents).expect("Error decrypting ascii85 text");
    println!("{}", String::from_utf8_lossy(bytes.as_slice()));
}
