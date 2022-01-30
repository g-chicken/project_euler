fn main() {
    let mut n: u64 = 1;

    loop {
        let mut ok: bool = true;

        for i in 2..7 {
            ok = is_permutations(n, n * i);

            if !ok {
                break;
            }
        }

        if ok {
            println!("{}", n);

            break;
        }

        n += 1;
    }
}

fn is_permutations(a: u64, b: u64) -> bool {
    let mut a_digit: Vec<u64> = get_digits_vec(a);
    let mut b_digit: Vec<u64> = get_digits_vec(b);

    a_digit.sort();
    b_digit.sort();

    a_digit == b_digit
}

fn get_digits_vec(num: u64) -> Vec<u64> {
    let mut n: u64 = num;
    let mut digits: Vec<u64> = Vec::<u64>::new();

    while n > 0 {
        digits.push(n % 10);
        n /= 10;
    }

    digits
}
