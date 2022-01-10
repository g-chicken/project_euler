// 1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 = 45
// so 9-digit pandigital is divided by 9.
fn main() {
    let primes: Vec<u64> = get_primes();
    let mut max: u64 = 0;
    let permutations: &mut Vec<u64> = &mut Vec::<u64>::new();
    let mut num: Vec<u8> = vec![1];

    for n in 2..9 {
        num.push(n);

        let num_for_permutations: &mut Vec<u8> = &mut Vec::<u8>::new();
        num_for_permutations.extend(num.iter().copied());

        permutations.clear();
        get_permutations(num_for_permutations, permutations, 0);

        for i in 0..permutations.len() {
            if permutations[i] % 2 == 0 {
                continue;
            }

            if is_prime(permutations[i], &primes) && permutations[i] > max {
                max = permutations[i];
            }
        }
    }

    println!("{}", max);
}

fn get_primes() -> Vec<u64> {
    let mut primes: Vec<u64> = Vec::<u64>::new();
    let mut num: u64 = 2;

    while (num as f64) < 100000000f64.sqrt() {
        if is_prime(num, &primes) {
            primes.push(num);
        }

        num += 1;
    }

    primes
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

fn get_permutations(num: &mut Vec<u8>, permutations: &mut Vec<u64>, p: u64) {
    if num.len() == 0 {
        if !permutations.contains(&p) {
            permutations.push(p);
        }

        return;
    }

    if num.iter().filter(|n| *n % 2 != 0).next() == None {
        return;
    }

    let num_len: usize = num.len();

    for i in 0..num_len {
        let n: u8 = num.remove(i);
        get_permutations(num, permutations, p * 10 + n as u64);
        num.insert(i, n);
    }
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
fn test_get_permutations() {
    let v: &mut Vec<u64> = &mut Vec::<u64>::new();
    get_permutations(&mut vec![1], v, 0);
    assert_eq!(vec![1], *v, "invalid (length 1)");

    v.clear();
    get_permutations(&mut vec![1, 2], v, 0);
    assert_eq!(vec![21], *v, "invalid (length 2)");

    v.clear();
    get_permutations(&mut vec![1, 2, 3], v, 0);
    assert_eq!(vec![123, 213, 231, 321], *v, "invalid (length 3)");
}
