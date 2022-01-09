fn main() {
    let primes: Vec<u32> = get_primes();
    let mut circular_primes: Vec<u32> = Vec::<u32>::new();
    let mut circular: u32 = 0;

    for prime in &primes {
        if *prime == 2u32 {
            circular += 1;

            continue;
        }

        if circular_primes.contains(&prime) {
            continue;
        }

        let mut is_even: bool = false;
        let mut circular_num: Vec<u32> = Vec::<u32>::new();
        let mut p: u32 = *prime;
        let digit_len: u32 = get_digit_len(*prime);

        for _i in 0..digit_len {
            if p % 2 == 0 {
                is_even = true;

                break;
            }

            if !circular_num.contains(&p) {
                circular_num.push(p);
            }

            p = p % 10u32.pow(digit_len - 1) * 10 + p / 10u32.pow(digit_len - 1);
        }

        if is_even {
            continue;
        }

        let mut is_circular_prime = true;
        for c in &circular_num {
            if !primes.contains(&c) {
                is_circular_prime = false;

                break;
            }
        }

        if is_circular_prime {
            circular += circular_num.len() as u32;
            circular_primes.append(&mut circular_num);
        }
    }

    println!("{}", circular);
}

fn get_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = Vec::<u32>::new();
    let mut num: u32 = 2;

    while num < 1000000 {
        let num_sqrt: f64 = (num as f64).sqrt();
        let mut is_prime: bool = true;

        for prime in &primes {
            if *prime as f64 > num_sqrt {
                break;
            }

            if num % *prime == 0 {
                is_prime = false;

                break;
            }
        }

        if is_prime {
            primes.push(num);
        }

        num += 1;
    }

    primes
}

fn get_digit_len(num: u32) -> u32 {
    let mut n: u32 = num;
    let mut len: u32 = 0;

    while n > 0 {
        len += 1;
        n /= 10;
    }

    len
}
