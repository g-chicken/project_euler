fn main() {
    let mut num = 3;
    let mut triangle_num = 3;
    let mut primes: Vec<u32> = vec![2, 3];

    loop {
        let mut n = *(primes.last().unwrap()) + 1;

        while primes.last().unwrap() * primes.last().unwrap() <= triangle_num {
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

        let mut factors: Vec<u32> = Vec::<u32>::with_capacity(primes.len()); 
        factors.resize(primes.len(), 0);

        let mut r = triangle_num;
        let mut out_of_primes_range = false;

        while r > 1 {
            let mut r_is_prime = true;

            for i in 0..primes.len() as usize {
                if r % primes[i] == 0 {
                    r /= primes[i];
                    factors[i] += 1;
                    r_is_prime = false;

                    break;
                }
            }

            if r_is_prime {
                out_of_primes_range = true;
                break;
            }
        }

        let mut divisors = 1;

        if out_of_primes_range {
            divisors += 1;
        }

        for f in &factors {
            if *f != 0 {
                divisors *= *f + 1;
            }
        }

        if divisors > 500 {
            println!("{}: {}", triangle_num, divisors);

            break;
        }

        triangle_num += num;
        num += 1;
    }
}
