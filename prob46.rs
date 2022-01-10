fn main() {
    let mut n: u64 = 9;
    let mut primes: Vec<u64> = vec![2];

    loop {
        if !is_odd_composite(n) {
            n += 2;

            continue;
        }

        let mut m: u64 = *primes.last().unwrap() + 1;

        while *primes.last().unwrap() < n {
            if is_prime(m, &primes) {
                primes.push(m);
            }

            m += 1;
        }

        let mut terminate: bool = true;
        let mut i: usize = 0;

        while i < primes.len() && primes[i] < n {
            if (n - primes[i]) % 2 != 0 {
                i += 1;

                continue;
            }

            if is_square((n - primes[i]) / 2) {
                terminate = false;

                break;
            }

            i += 1;
        }

        if terminate {
            println!("{}", n);

            break;
        }

        n += 2;
    }
}

fn is_odd_composite(num: u64) -> bool {
    if num % 2 == 0 {
        return false;
    }

    let mut odd: u64 = 3;

    while (odd as f64) <= (num as f64).sqrt() {
        if num % odd != 0 {
            odd += 2;

            continue;
        }

        let d: u64 = num / odd;
        if d % 2 != 0 && d > 1 {
            return true;
        }

        odd += 2;
    }

    false
}

fn is_prime(num: u64, primes: &Vec<u64>) -> bool {
    if num <= 1 {
        return false;
    }

    let num_sqrt = (num as f64).sqrt();
    let mut i: usize = 0;

    while i < primes.len() && primes[i] as f64 <= num_sqrt {
        if num % primes[i] == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn is_square(num: u64) -> bool {
    let r: f64 = (num as f64).sqrt();

    r == r.floor() && r == r.ceil()
}

#[test]
fn test_is_odd_composite() {
    assert!(is_odd_composite(9));
    assert!(is_odd_composite(15));
    assert!(is_odd_composite(21));
    assert!(is_odd_composite(25));
    assert!(is_odd_composite(27));
    assert!(is_odd_composite(33));

    assert!(!is_odd_composite(1));
    assert!(!is_odd_composite(2));
    assert!(!is_odd_composite(3));
    assert!(!is_odd_composite(4));
    assert!(!is_odd_composite(5));
    assert!(!is_odd_composite(6));
    assert!(!is_odd_composite(7));
    assert!(!is_odd_composite(8));
    assert!(!is_odd_composite(10));
    assert!(!is_odd_composite(13));
    assert!(!is_odd_composite(17));
    assert!(!is_odd_composite(23));
    assert!(!is_odd_composite(29));
    assert!(!is_odd_composite(31));
}

#[test]
fn test_is_prime() {
    let primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    assert!(is_prime(53, &primes), "expected true but false (53)");
    assert!(is_prime(2, &primes), "expected true but false (2)");
    assert!(is_prime(13, &primes), "expected true but false (13)");
    assert!(is_prime(97, &primes), "expected true but false (97)");

    assert!(!is_prime(1, &primes), "expected false but true (1)");
    assert!(!is_prime(0, &primes), "expected false but true (0)");
    assert!(!is_prime(98, &primes), "expected false but true (98)");

    let empty_primes: Vec<u64> = Vec::<u64>::new();
    assert!(is_prime(53, &empty_primes), "expected true but false (53)");
    assert!(is_prime(2, &empty_primes), "expected true but false (2)");
    assert!(is_prime(13, &empty_primes), "expected true but false (13)");
    assert!(is_prime(97, &empty_primes), "expected true but false (97)");
    assert!(is_prime(98, &empty_primes), "expected true but false (98)");

    assert!(!is_prime(1, &empty_primes), "expected false but true (1)");
    assert!(!is_prime(0, &empty_primes), "expected false but true (0)");
}

#[test]
fn test_is_square() {
    assert!(is_square(1));
    assert!(is_square(4));
    assert!(is_square(9));
    assert!(is_square(25));
    assert!(is_square(36));
    assert!(is_square(49));

    assert!(!is_square(2));
    assert!(!is_square(3));
    assert!(!is_square(5));
    assert!(!is_square(8));
    assert!(!is_square(10));
    assert!(!is_square(24));
    assert!(!is_square(26));
    assert!(!is_square(35));
    assert!(!is_square(37));
    assert!(!is_square(48));
    assert!(!is_square(50));
}
