use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("encrypted.txt").unwrap();
    let decrypted = decrypt(&contents);
    match decrypted {
        Ok(payload) => println!("{}", payload),
        Err(e) => panic!("{}", e),
    }
}
