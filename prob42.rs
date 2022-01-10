use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let words: Vec<String> = match read_file() {
        Ok(w) => w,
        Err(err) => panic!("fail to read file: {:?}", err),
    };

    let mut counter: u32 = 0;

    for word in &words {
        let val: u32 = calc_word_value(word.to_string());

        if is_triangle_numbers(val) {
            counter += 1;
        }
    }

    println!("{}", counter);
}

fn calc_word_value(word: String) -> u32 {
    let a_byte: u8 = String::from("A").as_bytes()[0];
    let mut sum: u32 = 0;

    for b in word.as_bytes() {
        sum += ((b - a_byte) as u32) + 1;
    }

    sum
}

fn is_triangle_numbers(num: u32) -> bool {
    let r: f64 = ((num * 8 + 1) as f64).sqrt();

    r == r.floor() && r == r.ceil()
}

fn read_file() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let mut words: Vec<String> = Vec::<String>::new();

    let buf = BufReader::new(File::open("./prob42.txt").expect("open failed"));

    for line in buf.lines() {
        let l = line.expect("lines failed");
        let split: Vec<&str> = l.split(',').collect();

        for s in split {
            words.push(s.trim_matches('"').to_string());
        }
    }

    Ok(words)
}

#[test]
fn test_calc_word_value() {
    assert_eq!(55, calc_word_value("SKY".to_string()));
    assert_eq!(1, calc_word_value("A".to_string()));
    assert_eq!(26, calc_word_value("Z".to_string()));
}

#[test]
fn test_is_triangle_numbers() {
    assert!(is_triangle_numbers(0), "expect true but not (0)");
    assert!(is_triangle_numbers(1), "expect true but not (1)");
    assert!(is_triangle_numbers(3), "expect true but not (3)");
    assert!(is_triangle_numbers(6), "expect true but not (6)");
    assert!(is_triangle_numbers(10), "expect true but not (10)");
    assert!(is_triangle_numbers(15), "expect true but not (15)");
    assert!(is_triangle_numbers(21), "expect true but not (21)");
    assert!(is_triangle_numbers(28), "expect true but not (28)");
    assert!(is_triangle_numbers(36), "expect true but not (36)");
    assert!(is_triangle_numbers(45), "expect true but not (45)");
    assert!(is_triangle_numbers(55), "expect true but not (55)");

    assert!(!is_triangle_numbers(2), "expect false but not (2)");
    assert!(!is_triangle_numbers(4), "expect false but not (4)");
    assert!(!is_triangle_numbers(5), "expect false but not (5)");
    assert!(!is_triangle_numbers(7), "expect false but not (7)");
    assert!(!is_triangle_numbers(9), "expect false but not (9)");
    assert!(!is_triangle_numbers(11), "expect false but not (11)");
    assert!(!is_triangle_numbers(14), "expect false but not (14)");
    assert!(!is_triangle_numbers(16), "expect false but not (16)");
    assert!(!is_triangle_numbers(20), "expect false but not (20)");
    assert!(!is_triangle_numbers(22), "expect false but not (22)");
    assert!(!is_triangle_numbers(27), "expect false but not (27)");
    assert!(!is_triangle_numbers(29), "expect false but not (29)");
    assert!(!is_triangle_numbers(35), "expect false but not (35)");
    assert!(!is_triangle_numbers(37), "expect false but not (37)");
    assert!(!is_triangle_numbers(44), "expect false but not (44)");
    assert!(!is_triangle_numbers(46), "expect false but not (46)");
    assert!(!is_triangle_numbers(54), "expect false but not (54)");
    assert!(!is_triangle_numbers(56), "expect false but not (56)");
}
