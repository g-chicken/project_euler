const LIMIT_N: u32 = 10_000_000;

fn main() {
    let mut min_n = 0;
    let mut min = f64::MAX;
    let primes = get_primes();

    for n in 2..LIMIT_N {
        let phi = phi_func(n, primes.clone());
        if is_permutation(n, phi) {
            //println!("{}, {}", n, phi);
            let eval = (n as f64) / (phi as f64);
            if eval < min {
                min = eval;
                min_n = n;
            }
        }
    }

    println!("{} ({})", min_n, min);
}

fn get_primes() -> Vec<u32> {
    let mut primes = vec![2, 3, 5, 7, 11, 13, 17, 19];
    let mut n = 23;

    while n * n <= LIMIT_N {
        let mut index = 0;
        let mut is_prime = true;
        let limit = (n as f64).sqrt();

        while index < primes.len() && is_prime && (primes[index] as f64) <= limit {
            if n % primes[index] == 0 {
                is_prime = false;
            }

            index += 1;
        }

        if is_prime {
            primes.push(n);
        }

        n += 2;
    }

    primes
}

fn phi_func(n: u32, primes: Vec<u32>) -> u32 {
    let mut factorization = Vec::<u32>::new();
    let mut index = 0;
    let mut a = n;

    while index < primes.len() && a > 1 {
        if a % primes[index] == 0 {
            if !factorization.contains(&primes[index]) {
                factorization.push(primes[index]);
            }

            a /= primes[index];
            index = 0;
        } else {
            index += 1;
        }
    }

    if a > 1 {
        factorization.push(a);
    }

    let mut phi = n as f64;
    for &prime in factorization.iter() {
        phi *= 1.0 - 1.0 / prime as f64;
    }

    phi as u32
}

fn is_permutation(n: u32, phi: u32) -> bool {
    let mut n_vec = get_digits_vec(n);
    let mut phi_vec = get_digits_vec(phi);

    if n_vec.len() != phi_vec.len() {
        return false;
    }

    n_vec.sort();
    phi_vec.sort();
    for i in 0..n_vec.len() {
        if n_vec[i] != phi_vec[i] {
            return false;
        }
    }

    true
}

fn get_digits_vec(n: u32) -> Vec<u32> {
    let mut a = n;
    let mut v = Vec::<u32>::new();

    while a > 0 {
        v.push(a % 10);
        a /= 10;
    }

    v
}
