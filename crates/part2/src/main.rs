use ascii85::decrypt;
use std::fs::read_to_string;

fn main() {
    let contents = read_to_string("payloads/part2.txt").unwrap();
    let bytes = decrypt(&contents).expect("Error decrypting ascii85 text");

    let filtered_bytes: Vec<u8> = bytes
        .iter()
        .filter(|x| {
            let bit = **x;
            let last_digit_1 = bit & 1 == 1;
            let mut num_ones = u8::count_ones(bit);
            if last_digit_1 {
                num_ones -= 1;
            }
            let is_num_ones_even = num_ones % 2 == 0;
            return (is_num_ones_even && !last_digit_1) || (!is_num_ones_even && last_digit_1);
        })
        .cloned()
        .collect();

    let mut final_bytes: Vec<u8> = Vec::new();
    let mut pos = 0;
    while pos < filtered_bytes.len() - 7 {
        let mut bit_strs = String::with_capacity(56);
        for i in 0..8 {
            let byte = filtered_bytes[pos + i];
            let bit_string = format!("{:08b}", byte);
            bit_strs.push_str(&bit_string[0..7]);
        }
        for i in 0..7 {
            let bit_pos = i * 8;
            let bit_string = &bit_strs[bit_pos..bit_pos + 8];
            let byte = u8::from_str_radix(bit_string, 2).unwrap();
            final_bytes.push(byte);
        }
        pos += 8;
    }

    let decrypted = String::from_utf8(final_bytes).unwrap();
    println!("{}", decrypted);
}
