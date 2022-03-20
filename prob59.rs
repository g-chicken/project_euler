use std::fs::read_to_string;

fn main() {
    let raw_codes: Vec<u8> = read_file();
    let mut word_counter: [[u32; 255]; 3] = [[0; 255]; 3];

    for i in 0..raw_codes.len() {
        word_counter[i % 3][raw_codes[i] as usize] += 1;
    }

    let mut secret: [u8; 3] = [0; 3];

    for i in 0..word_counter.len() {
        for j in 0..word_counter[i].len() {
            if word_counter[i][secret[i] as usize] < word_counter[i][j] {
                secret[i] = j as u8;
            }
        }
    }

    for i in 0..secret.len() {
        secret[i] = secret[i] ^ ' ' as u8;
    }

    let mut decrypted_codes: Vec<u8> = Vec::<u8>::new();
    let mut val: u32 = 0;

    for i in 0..raw_codes.len() {
        decrypted_codes.push(raw_codes[i] ^ secret[i % 3]);
        val += (raw_codes[i] ^ secret[i % 3]) as u32;
    }

    println!("{:?}", String::from_utf8(secret.to_vec()).unwrap());
    println!("{:?}", String::from_utf8(decrypted_codes).unwrap());
    println!("{}", val);
}

fn read_file() -> Vec<u8> {
    let mut content = read_to_string("./prob59.txt").unwrap();
    content.pop();

    let mut bytes: Vec<u8> = Vec::<u8>::new();

    for c in content.split(',') {
        match c.parse::<u8>() {
            Ok(b) => bytes.push(b),
            Err(err) => panic!("{:?} (code: {})", err, c),
        };
    }

    bytes
}
