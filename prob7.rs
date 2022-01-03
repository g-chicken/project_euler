fn main() {
    let mut num = 3;
    let mut primes: Vec<u32> = vec![2];

    while primes.len() < 10001 {
        let num_sqrt = (num as f64).sqrt();
        let mut is_prime = true;

        for prime in &primes {
            if (*prime as f64) > num_sqrt {
                break;
            }

            if num % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(num);
        }

        num += 1;
    }

    println!("{:?}", primes.pop().unwrap());
}
