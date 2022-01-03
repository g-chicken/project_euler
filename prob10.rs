fn main() {
    let mut num: u128 = 3;
    let mut sum: u128 = 2;
    let mut primes: Vec<u128> = vec![2];

    while num < 2000000 {
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
            sum += num;
        }

        num += 1;
    }

    println!("{:?}", sum);
}
