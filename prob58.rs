fn main() {
    let mut side_len: u32 = 1;
    let mut total_primes: u32 = 0;
    let mut total_diagonals: u32 = 1;
    let mut primes: Vec<u32> = vec![2, 3];

    loop {
        side_len += 2;
        total_diagonals += 4;

        let mut n: u32 = side_len * side_len;
        for _i in 0..4 {
            primes = update_primes(n, &primes);

            if is_prime(n, &primes) {
                total_primes += 1;
            }

            n -= side_len - 1;
        }

        if (total_primes as f64) / (total_diagonals as f64) < 0.10 {
            break;
        }
    }

    println!("{}", side_len);
}

fn update_primes(limit: u32, primes: &Vec<u32>) -> Vec<u32> {
    let mut last: u32 = *primes.last().unwrap();
    let mut n: u32 = last + 2;
    let mut result: Vec<u32> = Vec::<u32>::new();
    result.extend(primes.iter().copied());

    while last * last <= limit {
        if is_prime(n, &result) {
            result.push(n);
            last = n;
        }

        n += 2;
    }

    result
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
