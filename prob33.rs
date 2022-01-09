fn main() {
    let mut denominator: u32 = 1;
    let mut numerator: u32 = 1;

    for a in 10..100 {
        for b in 10..a {
            let common_num: u32 = match common_num(a as u32, b as u32) {
                Some(num) => num,
                None => continue,
            };

            if common_num == 0 {
                continue;
            }

            let canceled_a: u32 = cancel_num(a, common_num);
            let canceled_b: u32 = cancel_num(b, common_num);

            if canceled_a == 0 || canceled_b == 0 {
                continue;
            }

            let gcd: u32 = euclidean_algo(a as u32, b);
            let canceled_gcd: u32 = euclidean_algo(canceled_a, canceled_b);

            if a / gcd == canceled_a / canceled_gcd && b / gcd == canceled_b / canceled_gcd {
                println!(
                    "{} / {} -> {} / {} ({}) -> {} / {}",
                    b,
                    a,
                    canceled_b,
                    canceled_a,
                    canceled_gcd,
                    b / gcd,
                    a / gcd,
                );

                denominator *= a;
                numerator *= b;
            }
        }
    }

    println!("{}", denominator / euclidean_algo(denominator, numerator));
}

fn common_num(a: u32, b: u32) -> Option<u32> {
    let a_v: Vec<u32> = get_digit_vec(a);
    let b_v: Vec<u32> = get_digit_vec(b);

    for f in &a_v {
        if b_v.contains(f) {
            return Some(*f);
        }
    }

    None
}

fn cancel_num(num: u32, cancel: u32) -> u32 {
    let vec: Vec<u32> = get_digit_vec(num);
    let mut canceled_num: u32 = 0;

    for v in &vec {
        if *v == cancel {
            continue;
        }

        canceled_num = canceled_num * 10 + *v;
    }

    canceled_num
}

fn get_digit_vec(num: u32) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::<u32>::new();
    let mut r: u32 = num;

    while r > 0 {
        v.insert(0, r % 10);
        r /= 10;
    }

    v
}

fn euclidean_algo(a: u32, b: u32) -> u32 {
    let mut n: u32 = b;
    let mut r: u32 = a % b;

    while r > 0 {
        let m: u32 = n;
        n = r;
        r = m % n;
    }

    n
}
