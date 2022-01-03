use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = read_file();
    let num: [[u32; 50]; 100] = match result {
        Ok(num) => num,
        Err(error) => {
            panic!("fail to read file: {:?}", error)
        },
    };

    let mut digit = num[0].len() - 1;
    let mut over_sum: u32 = 0;
    let mut add_num: Vec<u32> = Vec::new();

    loop {
        let mut sum: u32 = 0;
        for i in 0..num.len() as usize {
            sum += num[i][digit];
        }

        add_num.insert(0, (sum + over_sum) % 10);
        over_sum = (over_sum + sum ) / 10;

        if digit == 0 {
            break;
        }

        digit -= 1;
    }

    let mut i = 0;
    let mut first_ten: u64 = over_sum as u64;
    while first_ten < 1000000000 && i < add_num.len() {
        first_ten = first_ten * 10 + (add_num[i] as u64);
        i += 1;
    }

    println!("{}", first_ten);
}

fn read_file() -> Result<[[u32; 50]; 100], Box<dyn std::error::Error>>{
    let mut num: [[u32; 50]; 100] = [[0; 50]; 100];
    let mut line_num = 0;

    let buf = BufReader::new(File::open("./prob13.txt").expect("open failed"));

    for line in buf.lines() {
        let mut digit = 0;

        for s in line.expect("fail to read line").chars() {
            num[line_num][digit] = s.to_digit(10).unwrap();
            digit += 1;
        }

        line_num += 1;
    }

    Ok(num)
}
