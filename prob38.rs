// 9999 * 2 = 19998
// - 9999: 4 digits
// - 19998: 5 digits
//   - 4 + 5 = 9 digits (OK)
// 10000 * 2 = 20000
// - 10000: 5 digits
// - 20000: 5 digits
//   - 5 + 5 = 10 digit (NG)
fn main() {
    let mut max: u64 = 0;

    for num in 1..10000 {
        let mut digit_len: u32 = 0;
        let mut n: u64 = 1;
        let mut concatenated_product: u64 = 0;

        while digit_len < 9 {
            let v: u64 = num * n;
            let l: u32 = get_digit_len(v);

            digit_len += l;
            n += 1;
            concatenated_product = concatenated_product * 10u64.pow(l) + v;
        }

        if digit_len != 9 {
            continue;
        }

        if !is_pandigital(concatenated_product) {
            continue;
        }

        if concatenated_product > max {
            max = concatenated_product;
        }
    }

    println!("{}", max);
}

fn get_digit_len(num: u64) -> u32 {
    if num == 0 {
        return 1;
    }

    let mut n: u64 = num;
    let mut len: u32 = 0;

    while n > 0 {
        len += 1;
        n /= 10;
    }

    len
}

fn is_pandigital(num: u64) -> bool {
    if get_digit_len(num) != 9 {
        return false;
    }

    let mut vec: Vec<u64> = Vec::<u64>::new();
    let mut n: u64 = num;

    while n > 0 {
        let v = n % 10;
        if v == 0 {
            return false;
        }

        if vec.contains(&v) {
            return false;
        }

        vec.push(v);
        n /= 10;
    }

    true
}

#[test]
fn test_get_digit_len() {
    assert_eq!(1, get_digit_len(3), "expected 1 but not (3)");
    assert_eq!(1, get_digit_len(0), "expected 0 but not (1)");
    assert_eq!(4, get_digit_len(1234), "expected 1 but not (0)");
}

#[test]
fn test_is_pandigital() {
    assert!(
        is_pandigital(123456789),
        "expected true but not (123456789)"
    );
    assert!(
        is_pandigital(987654321),
        "expected true but not (987654321)"
    );
    assert!(
        is_pandigital(438651927),
        "expected true but not (438651927)"
    );

    assert!(!is_pandigital(0), "expected false but not (0)");
    assert!(!is_pandigital(1), "expected false but not (1)");
    assert!(
        !is_pandigital(1000000000),
        "expected false but not (1000000000)"
    );
    assert!(
        !is_pandigital(123405678),
        "expected false but not (123405678)"
    );
    assert!(
        !is_pandigital(256345981),
        "expected false but not (256345981)"
    );
}
