fn main() {
    let primes: Vec<u32> = get_primes();
    let mut result_a: i32 = 0;
    let mut result_b: i32 = 0;
    let mut max: i32 = 0;

    for p in &primes {
        if *p > 1000 {
            break;
        }

        for a in -999..1000 {
            let n: i32 = formula(a, *p as i32, &primes);

            if n > max {
                max = n;
                result_a = a;
                result_b = *p as i32;
            }
        }
    }

    println!(
        "{} (a: {}, b: {}, n: {})",
        result_a * result_b,
        result_a,
        result_b,
        max
    );
}

fn get_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::<u32>::new();
    let mut n: u32 = 2;

    while n < 10000 {
        let n_sqrt = (n as f64).sqrt();
        let mut is_prime = true;

        for prime in &primes {
            if (*prime as f64) > n_sqrt {
                break;
            }

            if n % prime == 0 {
                is_prime = false;

                break;
            }
        }

        if is_prime {
            primes.push(n);
        }

        n += 1;
    }

    primes
}

fn formula(a: i32, b: i32, primes: &Vec<u32>) -> i32 {
    let mut n: i32 = 0;

    loop {
        if n as u32 > *primes.last().unwrap() {
            panic!("over primes vector!");
        }

        let v: i32 = n * n + a * n + b;

        if v <= 0 {
            return n;
        }

        if !primes.contains(&(v as u32)) {
            return n;
        }

        n += 1;
    }
}
