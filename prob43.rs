fn main() {
    let divisible_num: [u64; 7] = [17, 13, 11, 7, 5, 3, 2];
    let mut n: u64 = 1;
    let mut sum: u64 = 0;

    while divisible_num[0] * n < 1000 {
        let second_third_digit: u64 = divisible_num[0] * n / 10;
        let digits: &mut Vec<u64> = &mut Vec::<u64>::new();

        let (d, ok) = get_digits(divisible_num[0] * n);
        if !ok {
            n += 1;

            continue;
        }

        digits.extend(d.iter().copied());

        sum = create_divisivility_property(
            &divisible_num,
            1,
            digits,
            second_third_digit,
            divisible_num[0] * n,
            sum,
        );

        n += 1;
    }

    println!("{}", sum);
}

fn create_divisivility_property(
    divisible_num: &[u64; 7],
    index: usize,
    digits: &mut Vec<u64>,
    first_second_digits: u64,
    num: u64,
    sum: u64,
) -> u64 {
    let mut s: u64 = sum;

    if divisible_num.len() == index {
        for n in 1..10 as u64 {
            if !digits.contains(&n) {
                return s + n * 10u64.pow((index + 2) as u32) + num;
            }
        }

        return s;
    }

    for third_digit in 0..10 as u64 {
        if digits.contains(&third_digit) {
            continue;
        }

        let v: u64 = third_digit * 100 + first_second_digits;

        if v % divisible_num[index] == 0 {
            digits.push(third_digit);
            s = create_divisivility_property(
                divisible_num,
                index + 1,
                digits,
                v / 10,
                third_digit * 10u64.pow((index + 2) as u32) + num,
                s,
            );
            digits.pop();
        }
    }

    s
}

fn get_digits(num: u64) -> (Vec<u64>, bool) {
    if num == 0 {
        return (vec![0], true);
    }

    let mut n: u64 = num;
    let mut digits: Vec<u64> = Vec::<u64>::new();
    let mut is_duplicated: bool = false;

    while n > 0 {
        is_duplicated = is_duplicated || digits.contains(&(n % 10));
        digits.push(n % 10);
        n /= 10;
    }

    (digits, !is_duplicated)
}
