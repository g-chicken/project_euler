fn main() {
    let mut digits: Vec<u32> = vec![1];

    for _i in 0..1000 {
        let mut num: u32 = 0;
        let mut j = digits.len() - 1;

        loop {
            let n = digits[j] * 2;

            digits[j] = (n + num) % 10;
            num = (n + num) / 10;

            if j == 0 {
                while num > 0 {
                    digits.insert(0, num % 10);
                    num /= 10;
                }

                break;
            }

            j -= 1;
        }
    }

    let mut digit_sum: u32 = 0;

    for d in &digits {
        digit_sum += *d
    }

    println!("{}", digit_sum);
}
