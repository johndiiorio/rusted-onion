mod errors;

pub use errors::Ascii85Error;

pub fn decrypt(p: &str) -> Result<Vec<u8>, Ascii85Error> {
    if !p.starts_with("<~") {
        return Err(Ascii85Error::InvalidFormat {
            message: String::from("payload does not end with <~"),
        });
    }
    if !p.ends_with("~>") {
        return Err(Ascii85Error::InvalidFormat {
            message: String::from("payload does not end with ~>"),
        });
    }
    let mut payload = p
        .replace("\n", "")
        .replace(" ", "")
        .replace("<~", "")
        .replace("~>", "");

    // Pad payload with 'u' chars to fill final block of 5
    let mut num_u_to_add = 0;
    let payload_remainder = payload.len() % 5;
    if payload_remainder > 0 {
        num_u_to_add = 5 - payload_remainder;
    }
    for _ in 0..num_u_to_add {
        payload.push('u');
    }

    let mut decrypted_chunks = Vec::with_capacity(payload.len() / 5);
    let mut iter = payload.chars();
    let mut pos = 0;

    while pos < payload.len() {
        let mut len = 0;
        for c in iter.by_ref().take(5) {
            len += c.len_utf8();
        }
        let chunk = &payload[pos..pos + len];
        let bit_number: usize = (0..5)
            .into_iter()
            .map(|i| {
                let ascii_85_value = chunk.chars().nth(i).unwrap() as usize - 33;
                return ascii_85_value * 85_usize.pow(4 - i as u32);
            })
            .sum::<usize>();

        let mut binary = format!("{:b}", bit_number);
        let num_zeros_to_add = 32 - binary.len();
        for _ in 0..num_zeros_to_add {
            binary.insert(0, '0');
        }

        let mut decrypted_chunk = decrypt_binary_str(&binary, 8);
        decrypted_chunks.append(&mut decrypted_chunk);
        pos += len;
    }

    return Ok(decrypted_chunks);
}

fn decrypt_binary_str(string: &str, sub_len: usize) -> Vec<u8> {
    let mut decrypted = Vec::with_capacity(string.len() / sub_len);
    let mut iter = string.chars();
    let mut pos = 0;

    while pos < string.len() {
        let mut len = 0;
        for ch in iter.by_ref().take(sub_len) {
            len += ch.len_utf8();
        }
        let bits = &string[pos..pos + len];
        decrypted.push(u8::from_str_radix(bits, 2).unwrap());
        pos += len;
    }
    return decrypted;
}
