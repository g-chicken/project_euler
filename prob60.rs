use std::collections::HashMap;

const LIMIT: u32 = 10000;
const PRIME_NUM: usize = 5;

fn main() {
    let primes: Vec<u32> = get_primes();
    let map: HashMap<u32, Vec<u32>> = get_prime_map(&primes);
    let mut keys = map.keys().collect::<Vec<_>>();

    keys.sort();

    for key in keys.iter() {
        let mut result: Vec<u32> = vec![**key];

        get_result(&key, &mut result, &map);

        if result.len() >= PRIME_NUM {
            println!("{} ({:?})", result.iter().sum::<u32>(), result);

            break;
        }
    }
}

fn get_result(key: &u32, result: &mut Vec<u32>, map: &HashMap<u32, Vec<u32>>) {
    for p in &map[key] {
        let mut ok: bool = true;

        for r in result.iter() {
            match map[r].binary_search(&p) {
                Ok(_) => {}
                Err(_) => ok = false,
            }

            if !ok {
                break;
            }
        }

        if !ok {
            continue;
        }

        result.push(*p);

        if result.len() >= PRIME_NUM {
            return;
        }

        match map.get(p) {
            Some(_) => get_result(&p, result, map),
            None => {}
        }

        if result.len() >= PRIME_NUM {
            return;
        }

        result.pop();
    }
}

fn get_prime_map(primes: &Vec<u32>) -> HashMap<u32, Vec<u32>> {
    let mut map: HashMap<u32, Vec<u32>> = HashMap::new();

    for i in 0..primes.len() - 1 {
        for j in i + 1..primes.len() {
            let p: u32 = primes[i];
            let q: u32 = primes[j];

            if is_prime(join_number(p, q), &primes) && is_prime(join_number(q, p), &primes) {
                match map.get(&p) {
                    None => {
                        map.insert(p, vec![q]);
                    }
                    Some(v) => {
                        let mut vec: Vec<u32> = v.to_vec();
                        vec.push(q);
                        map.insert(p, vec);
                    }
                }
            }
        }
    }

    map
}

fn get_primes() -> Vec<u32> {
    let mut primes: Vec<u32> = vec![2, 3];
    let mut n: u32 = 5;

    while n < LIMIT {
        if is_prime(n, &primes) {
            primes.push(n);
        }

        n += 2;
    }

    primes
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

fn join_number(left: u32, right: u32) -> u32 {
    let mut digit: u32 = 0;
    let mut r: u32 = right;

    while r > 0 {
        digit += 1;
        r /= 10;
    }

    left * 10u32.pow(digit) + right
}

#[test]
fn test_join_number() {
    assert_eq!(join_number(1, 2), 12);
    assert_eq!(join_number(10, 2), 102);
    assert_eq!(join_number(1234, 2), 12342);
    assert_eq!(join_number(1, 20), 120);
    assert_eq!(join_number(15, 234), 15234);
    assert_eq!(join_number(478, 1032), 4781032);
}
