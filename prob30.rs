fn main() {
    // 9^5 * 7 = 413,343, so search for number less than 1,000,000

    let mut sum: u32 = 0;

    for n in 2..1000000 {
        if n == sum_fifth_powers_of_digits(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn sum_fifth_powers_of_digits(num: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut r: u32 = num;

    while r > 0 {
        let n: u32 = r % 10;
        r /= 10;

        sum += n.pow(5);
    }

    sum
}
