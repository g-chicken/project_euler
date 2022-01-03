use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = read_file();
    let grids = match result {
        Ok(grids) => grids,
        Err(error) => {
            panic!("fail to read file: {:?}", error)
        },
    };

    let mut max: u32 = 0;

    for row in 0..20 {
        for col in 0..20 {
            if row >= 3  && col + 3 < grids[row].len() {
                let mut n: u32 = 1;

                for i in 0..4 {
                    n *= grids[row-i][col+i];
                }

                if n > max {
                    max = n;
                }
            }

            if col + 3 < grids.len() {
                let mut n: u32 = 1;

                for i in 0..4 {
                    n *= grids[row][col+i];
                }

                if n > max {
                    max = n;
                }
            }

            if row + 3 < grids.len() && col + 3 < grids[row].len() {
                let mut n: u32 = 1;

                for i in 0..4 {
                    n *= grids[row+i][col+i];
                }

                if n > max {
                    max = n;
                }
            }

            if row + 3 < grids.len() {
                let mut n: u32 = 1;

                for i in 0..4 {
                    n *= grids[row+i][col];
                }

                if n > max {
                    max = n;
                }
            }
        }
    }

    println!("max: {}", max);
}

fn read_file() -> Result<[[u32; 20]; 20], Box<dyn std::error::Error>>{
    let mut grids: [[u32; 20]; 20] = [[0; 20]; 20];
    let mut row = 0;

    let buf = BufReader::new(File::open("./prob11.txt").expect("open failed"));

    for line in buf.lines() {
        let l = line.expect("lines failed");
        let split: Vec<&str> = l.split(' ').collect();
        let mut col = 0;

        for s in split {
            grids[row][col] = s.parse().unwrap();
            col += 1;
        }

        row += 1;
    }

    Ok(grids)
}
