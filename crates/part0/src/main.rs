use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("part0.txt").unwrap();
    let decrypted = decrypt(&contents).expect("Error decrypting ascii85 text");
    println!("{}", decrypted);
}
