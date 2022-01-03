use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let result = read_file();
    let num: [u32; 5050];
    let first_child_index: [usize; 5050];

    match result {
        Ok((n, child)) => {
            num = n;
            first_child_index = child;
        }
        Err(error) => {
            panic!("fail to read file: {:?}", error)
        }
    };

    let mut dp: [u32; 5050] = [0; 5050];
    let mut index = dp.len() - 1;

    loop {
        if first_child_index[index] >= num.len() {
            dp[index] = num[index];
            index -= 1;

            continue;
        }

        let mut max = dp[first_child_index[index]];

        if dp[first_child_index[index] + 1] > max {
            max = dp[first_child_index[index] + 1];
        }

        dp[index] = num[index] + max;

        if index == 0 {
            break;
        }

        index -= 1;
    }

    println!("{}", dp[0]);
}

fn read_file() -> Result<([u32; 5050], [usize; 5050]), Box<dyn std::error::Error>> {
    let mut num: [u32; 5050] = [0; 5050];
    let mut child_index: [usize; 5050] = [0; 5050];
    let mut row = 1;
    let mut index = 0;

    let buf = BufReader::new(File::open("./prob67.txt").expect("open failed"));

    for line in buf.lines() {
        let l = line.expect("lines failed");
        let split: Vec<&str> = l.split(' ').collect();

        for s in split {
            num[index] = s.parse().unwrap();
            child_index[index] = index + row;
            index += 1;
        }

        row += 1;
    }

    Ok((num, child_index))
}
