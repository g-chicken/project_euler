use std::collections::HashMap;

const PERMUTATIONS: u64 = 5;

fn main() {
    let mut min_cube_hash: HashMap<u64, u64> = HashMap::new();
    let mut cube_digit_count_hash: HashMap<u64, u64> = HashMap::new();
    let mut base: u64 = 1;

    loop {
        let cube = base.pow(3);
        let digit_numbers = count_digit_number(cube);
        //println!("{} - {}", base, cube);

        let exist_count_some = cube_digit_count_hash.get(&digit_numbers);
        if exist_count_some == None {
            min_cube_hash.insert(digit_numbers, cube);
            cube_digit_count_hash.insert(digit_numbers, 1);
        } else {
            let exist_count = *exist_count_some.unwrap();
            if exist_count + 1 == PERMUTATIONS as u64 {
                println!("{}", min_cube_hash.get(&digit_numbers).unwrap());
                return;
            }

            cube_digit_count_hash.insert(digit_numbers, exist_count + 1);
        }

        base += 1;
    }
}

fn count_digit_number(n: u64) -> u64 {
    let mut muted_n = n;
    let mut digit_count: [u64; 10] = [0; 10];

    while muted_n > 0 {
        digit_count[(muted_n % 10) as usize] += 1;
        muted_n /= 10;
    }

    let mut num: u64 = 0;
    for i in 0..digit_count.len() {
        num += digit_count[i] * 10u64.pow(i as u32);
    }

    num
}
