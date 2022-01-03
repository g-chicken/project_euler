use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = read_file();
    let digits = match result {
        Ok(digits) => digits,
        Err(error) => {
            panic!("fail to read file: {:?}", error)
        },
    };

    let mut max: u128 = 0;
    for i in 0..987 {
        let mut num: u128 = 1;

        for j in 0..13 {
            num *= digits[i+j];
        }

        if num > max {
            max = num;
        }
    }

    println!("max: {}", max);
}

fn read_file() -> Result<[u128; 1000], Box<dyn std::error::Error>>{
    let mut digits: [u128; 1000] = [0; 1000];
    let mut index = 0;

    let buf = BufReader::new(File::open("./prob8.txt").expect("open failed"));

    for line in buf.lines() {
        for c in line.expect("lines failed").chars() {
            digits[index] = (c.to_digit(10).unwrap() as u128);
            index += 1;
        }
    }

    Ok(digits)
}
