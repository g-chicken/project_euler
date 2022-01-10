fn main() {
    let mut n: u64 = 1;
    let mut counter: u8 = 0;

    while counter < 3 {
        let hexagonal = n * (2 * n - 1);

        if is_pentagonal(hexagonal) {
            println!("{}", hexagonal);
            counter += 1;
        }

        n += 1;
    }
}

fn is_pentagonal(num: u64) -> bool {
    let r: f64 = ((num * 24 + 1) as f64).sqrt();

    r == r.floor() && r == r.ceil() && (r as u64 + 1) % 6 == 0
}

#[test]
fn test_is_pentagonal() {
    assert!(is_pentagonal(1));
    assert!(is_pentagonal(5));
    assert!(is_pentagonal(12));
    assert!(is_pentagonal(22));
    assert!(is_pentagonal(35));

    assert!(!is_pentagonal(2));
    assert!(!is_pentagonal(4));
    assert!(!is_pentagonal(6));
    assert!(!is_pentagonal(11));
    assert!(!is_pentagonal(13));
    assert!(!is_pentagonal(21));
    assert!(!is_pentagonal(23));
    assert!(!is_pentagonal(34));
    assert!(!is_pentagonal(36));
}
