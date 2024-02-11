use std::collections::HashMap;

fn main() {
    let mut dp = HashMap::<u32, HashMap<u32, u32>>::new();
    let mut primes: Vec<u32> = vec![2, 3, 5, 7];
    let mut n = 2;

    loop {
        insert_new_primes(&mut primes, n);
        let mut index = 0;
        let mut target = n - primes[index];
        let mut dp_map = HashMap::<u32, u32>::new();

        while index < primes.len() && target >= primes[index] {
            let wrapped_map = dp.get(&target);
            if wrapped_map != None {
                let mut count = 0;
                let mut ok_insert = false;
                for (&key, &val) in wrapped_map.unwrap() {
                    if key >= primes[index] {
                        ok_insert = true;
                        if val == 0 {
                            count += 1;
                        } else {
                            count += val;
                        }
                    }
                }

                if ok_insert {
                    dp_map.insert(primes[index], count);
                }
            }

            index += 1;
            target = n - primes[index];
        }

        if primes.contains(&n) {
            dp_map.insert(n, 0);
        }

        dp.insert(n, dp_map.clone());

        let mut count = 0;
        for &val in dp_map.values() {
            count += val;
        }

        if count > 5000 {
            break;
        }
        n += 1;
    }

    //for (key, val) in &dp {
    //    println!("{:>3} : {:?}", key, val);
    //}

    println!("{}", n);
}

fn insert_new_primes(primes: &mut Vec<u32>, limit: u32) {
    let mut n = *primes.last().unwrap();

    while n <= limit - 2 {
        n += 2;

        let mut i = 0;
        let mut is_prime = true;

        while i < primes.len() && primes[i] * primes[i] <= n && is_prime {
            is_prime = n % primes[i] != 0;
            i += 1;
        }

        if is_prime {
            primes.push(n);
        }
    }
}
