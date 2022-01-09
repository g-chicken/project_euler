fn main() {
    let mut primes: Vec<u32> = vec![2];
    let mut n: u32 = 3;
    let mut sum: u32 = 0;
    let mut truncatable_prime_num: u8 = 0;

    while truncatable_prime_num < 11 {
        if !is_prime(n, &primes) {
            n += 2;

            continue;
        }

        primes.push(n);

        if n <= 7 {
            n += 2;

            continue;
        }

        let mut is_truncatable_prime: bool = true;

        let mut left_truncated: u32 = n / 10;
        while left_truncated > 0 {
            if left_truncated != 2 && left_truncated % 2 == 0 {
                is_truncatable_prime = false;

                break;
            }

            if !primes.contains(&left_truncated) {
                is_truncatable_prime = false;

                break;
            }

            left_truncated /= 10;
        }

        if !is_truncatable_prime {
            n += 2;

            continue;
        }

        let mut digit_len: u32 = get_digit_len(n) - 1;
        let mut right_truncated: u32 = n % 10u32.pow(digit_len);
        while right_truncated > 0 {
            if right_truncated != 2 && right_truncated % 2 == 0 {
                is_truncatable_prime = false;

                break;
            }

            if !primes.contains(&right_truncated) {
                is_truncatable_prime = false;

                break;
            }

            digit_len -= 1;
            right_truncated %= 10u32.pow(digit_len);
        }

        if !is_truncatable_prime {
            n += 2;

            continue;
        }

        truncatable_prime_num += 1;
        sum += n;
        n += 2;
    }

    println!("{}", sum);
}

fn is_prime(num: u32, primes: &Vec<u32>) -> bool {
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

fn get_digit_len(num: u32) -> u32 {
    let mut digit_len: u32 = 0;
    let mut n: u32 = num;

    while n > 0 {
        digit_len += 1;
        n /= 10;
    }

    digit_len
}

#[test]
fn test_is_prime() {
    let primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    assert!(is_prime(53, &primes), "expected true but false (53)");
    assert!(is_prime(2, &primes), "expected true but false (2)");
    assert!(is_prime(13, &primes), "expected true but false (13)");
    assert!(is_prime(97, &primes), "expected true but false (97)");

    assert!(!is_prime(1, &primes), "expected false but true (1)");
    assert!(!is_prime(0, &primes), "expected false but true (0)");
    assert!(!is_prime(98, &primes), "expected false but true (98)");

    let empty_primes: Vec<u32> = Vec::<u32>::new();
    assert!(is_prime(53, &empty_primes), "expected true but false (53)");
    assert!(is_prime(2, &empty_primes), "expected true but false (2)");
    assert!(is_prime(13, &empty_primes), "expected true but false (13)");
    assert!(is_prime(97, &empty_primes), "expected true but false (97)");
    assert!(is_prime(98, &empty_primes), "expected true but false (98)");

    assert!(!is_prime(1, &empty_primes), "expected false but true (1)");
    assert!(!is_prime(0, &empty_primes), "expected false but true (0)");
}

#[test]
fn test_get_digit_len() {
    assert_eq!(1, get_digit_len(3), "expected 1 but not (3)");
    assert_eq!(0, get_digit_len(0), "expected 0 but not (0)");
    assert_eq!(4, get_digit_len(1234), "expected 1 but not (0)");
}
