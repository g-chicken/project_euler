use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = read_file();
    let mut names: Vec<String>;

    match result {
        Ok(n) => {
            names = n;
        }
        Err(error) => {
            panic!("fail to read file: {:?}", error)
        }
    };

    names.sort();

    let a_byte: u8 = String::from("A").as_bytes()[0];
    let mut sum: u32 = 0;

    for i in 0..names.len() as usize {
        let mut worth: u32 = 0;

        for b in names[i].as_bytes() {
            worth += ((b - a_byte) as u32) + 1;
        }

        sum += (i as u32 + 1) * worth;
    }

    println!("{}", sum);
}

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut names: Vec<String> = Vec::<String>::new();

    let buf = BufReader::new(File::open("./prob22.txt").expect("open failed"));

    for line in buf.lines() {
        let l = line.expect("lines failed");
        let split: Vec<&str> = l.split(',').collect();

        for s in split {
            names.push(s.trim_matches('"').to_string());
        }
    }

    Ok(names)
}
