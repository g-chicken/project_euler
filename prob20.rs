fn main() {
    let mut digits: Vec<u32> = vec![1];

    for i in 1..101 {
        let mut num: u32 = 0;
        let mut j = digits.len() - 1;

        loop {
            let n = digits[j] * i;

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
