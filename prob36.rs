fn main() {
    let mut sum: u32 = 0;

    for n in 1..1000000 {
        if is_decimal_base_10(n) && is_decimal_base_2(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn is_decimal_base_10(num: u32) -> bool {
    let mut r: u32 = num;
    let mut n: u32 = 0;

    while r > 0 {
        n = n * 10 + r % 10;
        r /= 10;
    }

    num == n
}

fn is_decimal_base_2(num: u32) -> bool {
    let mut base2: Vec<u8> = Vec::<u8>::new();
    let mut n: u32 = num;

    while n > 0 {
        base2.push((n % 2) as u8);
        n /= 2;
    }

    let mut reversed_base2: Vec<u8> = Vec::<u8>::new();
    reversed_base2.extend(base2.iter().copied());
    reversed_base2.reverse();

    base2 == reversed_base2
}

#[test]
fn test_is_decimal_base_10() {
    assert!(is_decimal_base_10(585), "expected true but not (585)");
    assert!(is_decimal_base_10(5), "expected true but not (5)");
    assert!(is_decimal_base_10(0), "expected true but not (0)");
    assert!(is_decimal_base_10(11), "expected true but not (11)");
    assert!(is_decimal_base_10(1001), "expected true but not (1001)");
    assert!(is_decimal_base_10(10801), "expected true but not (10801)");

    assert!(!is_decimal_base_10(10), "expected false but not (10)");
    assert!(!is_decimal_base_10(38), "expected false but not (38)");
    assert!(
        !is_decimal_base_10(101010),
        "expected false but not (101010)"
    );
}

#[test]
fn test_is_decimal_base_2() {
    assert!(is_decimal_base_2(585), "expected true but not (585)");
    assert!(is_decimal_base_2(0), "expected true but not (0)");
    assert!(is_decimal_base_2(1), "expected true but not (1)");
    assert!(is_decimal_base_2(21), "expected true but not (21)");

    assert!(!is_decimal_base_2(2), "expected false but not (2)");
    assert!(!is_decimal_base_2(10), "expected false but not (10)");
}
