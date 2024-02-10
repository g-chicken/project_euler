const LIMIT_N: u32 = 1_000_000;

fn main() {
    let mut min_n = 0;
    let mut min = f64::MAX;
    let primes = get_primes();

    for n in 2..(LIMIT_N + 1) {
        let phi = phi_func(n, primes.clone());
        //println!("{}, {}", n, phi);
        if min > phi {
            min_n = n;
            min = phi;
        }

        //if n % 1000 == 0 {
        //    println!("{} : {} ({})", n, min_n, min);
        //}
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

fn phi_func(n: u32, primes: Vec<u32>) -> f64 {
    let mut factorization = Vec::<u32>::new();
    let mut index = 0;
    let mut a = n;

    while index < primes.len() && a > 1 {
        //println!("{}: {}", n, a);
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

    let mut phi = 1.0;
    for &prime in factorization.iter() {
        phi *= 1.0 - 1.0 / prime as f64;
    }

    phi
}
