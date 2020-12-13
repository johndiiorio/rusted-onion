use ascii85::decrypt as decrypt_ascii;
use std::convert::TryInto;
use std::fs::read_to_string;

use openssl::aes::{unwrap_key, AesKey};
use openssl::symm::{decrypt as decrypt_aes, Cipher};

fn main() {
    let contents = read_to_string("payloads/part5.txt").unwrap();
    let bytes = decrypt_ascii(&contents).expect("Error decrypting ascii85 text");

    let mut key = [0u8; 32];
    unwrap_key(
        &AesKey::new_decrypt(&bytes[0..32]).unwrap(),
        Some(bytes[32..40].try_into().unwrap()),
        &mut key,
        &bytes[40..80],
    )
    .unwrap();

    let cipher = Cipher::aes_256_cbc();
    let decrypted = decrypt_aes(
        cipher,
        &key[..],
        Some(&bytes[80..96]),
        &mut bytes[96..].to_owned()[..],
    )
    .unwrap();

    let decrypted_str = String::from_utf8(decrypted).unwrap();
    println!("{}", decrypted_str);
}
