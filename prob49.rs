fn main() {
    let primes: Vec<u32> = get_4digit_primes();

    for i in 0..primes.len() - 1 {
        let a: u32 = primes[i];

        for j in (i + 1)..primes.len() {
            let b: u32 = primes[j];

            if !is_permutations(a, b) {
                continue;
            }

            let d: u32 = b - a;
            if !primes.contains(&(b + d)) || !is_permutations(b, b + d) {
                continue;
            }

            println!("{}{}{} ({} - {} - {})", a, b, b + d, a, b, b + d);
        }
    }
}

fn get_4digit_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2];
    let mut num: u32 = 3;

    while num < 10000 {
        if is_prime(num, &primes) {
            primes.push(num);
        }

        num += 1;
    }

    primes.iter().cloned().filter(|&p| p >= 1000).collect()
}

fn is_prime(num: u32, primes: &Vec<u32>) -> bool {
    let num_sqrt: f64 = (num as f64).sqrt();
    let mut i: usize = 0;

    while i < primes.len() && primes[i] as f64 <= num_sqrt {
        if num % primes[i] == 0 {
            return false;
        }

        i += 1;
    }

    true
}

fn is_permutations(a: u32, b: u32) -> bool {
    let mut a_digit: Vec<u32> = get_digits_vec(a);
    let mut b_digit: Vec<u32> = get_digits_vec(b);

    a_digit.sort();
    b_digit.sort();

    a_digit == b_digit
}

fn get_digits_vec(num: u32) -> Vec<u32> {
    let mut n: u32 = num;
    let mut digits: Vec<u32> = Vec::<u32>::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}

#[test]
fn test_is_prime() {
    let primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    assert!(is_prime(3, &primes));
    assert!(is_prime(29, &primes));
    assert!(is_prime(53, &primes));
    assert!(is_prime(67, &primes));
    assert!(is_prime(71, &primes));
    assert!(is_prime(83, &primes));
    assert!(is_prime(97, &primes));
    assert!(is_prime(373, &primes));

    assert!(is_prime(53 * 53, &primes));

    assert!(!is_prime(4, &primes));
    assert!(!is_prime(39, &primes));
    assert!(!is_prime(57, &primes));
}

#[test]
fn test_is_permutations() {
    assert!(is_permutations(1, 1));
    assert!(is_permutations(123, 231));
    assert!(is_permutations(98, 98));
    assert!(is_permutations(1223, 2312));
    assert!(is_permutations(2112, 1221));
    assert!(is_permutations(1111, 1111));

    assert!(!is_permutations(2231, 1131));
    assert!(!is_permutations(10, 1));
    assert!(!is_permutations(2, 3));
    assert!(!is_permutations(1032, 132));
}
