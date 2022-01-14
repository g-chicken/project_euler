fn main() {
    let mut primes: Vec<u32> = vec![2];
    let mut num: u32 = 2;
    let mut sequence: u8 = 0;
    let result: u8 = 4;

    loop {
        let num_sqrt: f64 = (num as f64).sqrt();
        let mut last: u32 = *(primes.last().unwrap());

        while (*(primes.last().unwrap()) as f64) <= num_sqrt {
            if is_prime(last, &primes) {
                primes.push(last);
            }

            last += 1;
        }

        if is_prime(num, &primes) {
            primes.push(num);
        }

        if get_prime_factor_num(num, &primes) == result {
            sequence += 1;
        } else {
            sequence = 0;
        }

        if sequence == result {
            break;
        }

        num += 1;
    }

    println!("{} - {} - {} - {}", num - 3, num - 2, num - 1, num);
}

fn get_prime_factor_num(num: u32, primes: &Vec<u32>) -> u8 {
    let mut n: u32 = num;
    let mut divided_primes: Vec<u32> = Vec::<u32>::new();
    let mut i: usize = 0;

    while n > 1 {
        if i > primes.len() {
            panic!("out of primes range (num: {})", num);
        }

        if n % primes[i] == 0 {
            if !divided_primes.contains(&primes[i]) {
                divided_primes.push(primes[i]);
            }

            n /= primes[i];
            i = 0;

            continue;
        }

        i += 1;
    }

    divided_primes.len() as u8
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
fn test_get_prime_factor_num() {
    let primes: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    assert_eq!(1, get_prime_factor_num(2, &primes), "2");
    assert_eq!(1, get_prime_factor_num(3, &primes), "3");
    assert_eq!(1, get_prime_factor_num(4, &primes), "4");
    assert_eq!(1, get_prime_factor_num(5, &primes), "5");
    assert_eq!(2, get_prime_factor_num(6, &primes), "6");
    assert_eq!(2, get_prime_factor_num(24, &primes), "24");
    assert_eq!(3, get_prime_factor_num(30, &primes), "30");
    assert_eq!(4, get_prime_factor_num(210, &primes), "210");
}
