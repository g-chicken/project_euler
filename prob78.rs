use std::collections::HashMap;

const MAX_DIGIT: u64 = 1_000_000;

fn main() {
    let mut dp = HashMap::<u128, HashMap<u128, u64>>::new();
    let mut n = 1;

    loop {
        let mut target = n - n / 2;
        let mut dp_map = HashMap::<u128, u64>::new();
        let mut previous_count = 1;

        while target < n {
            let wrapped_map = dp.get(&target);
            if wrapped_map != None {
                let mut key = n - target;
                let mut wrapped_count = wrapped_map.unwrap().get(&key);
                while wrapped_count == None && key <= target {
                    key += 1;
                    wrapped_count = wrapped_map.unwrap().get(&key);
                }

                let count = (wrapped_count.unwrap() + previous_count) % MAX_DIGIT;
                dp_map.insert(n - target, count);
                previous_count = count;
            }
            target += 1;
        }

        dp_map.insert(n, 1);
        dp.insert(n, dp_map);

        //let mut collect: Vec<(&u128, &u64)> = dp_map.iter().collect();
        //collect.sort_by(|a, b| a.0.cmp(b.0));
        //println!("{:>3} : {:?} ({})", n, collect, count);
        //println!("{:>9}: {:>6}", n, count);

        if previous_count % MAX_DIGIT == 0 {
            break;
        }

        if n % 1000 == 0 {
            println!("done {}", n);
        }

        n += 1;
    }

    println!("{}", n);
}
