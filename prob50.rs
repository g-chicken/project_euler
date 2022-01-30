fn main() {
    let primes: Vec<u32> = get_primes_below_million();
    let mut max_term: u32 = 1;
    let mut max_term_prime: u32 = 2;
    let mut dp: Vec<u32> = vec![2];

    for i in 1..primes.len() {
        let mut new_dp: Vec<u32> = vec![primes[i]];

        for v in &dp {
            new_dp.push(v + primes[i]);
        }

        dp.clear();

        for j in 0..new_dp.len() {
            if new_dp[j] >= 1000000 {
                break;
            }

            dp.push(new_dp[j]);

            if is_prime(new_dp[j], &primes) && max_term < (j + 1) as u32 {
                max_term = (j + 1) as u32;
                max_term_prime = dp[j];
            }
        }
    }

    println!("{} (term: {})", max_term_prime, max_term);
}

fn get_primes_below_million() -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2];
    let mut num: u32 = 3;

    while num < 1000000 {
        if is_prime(num, &primes) {
            primes.push(num);
        }

        num += 2;
    }

    primes
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
