fn main() {
    let mut sum1: u64 = 0;
    let mut sum2: u64 = 0;

    for n in 1..101 {
        sum1 += n * n;
        sum2 += n;
    }

    println!("{} - {} = {}", sum2*sum2, sum1, sum2*sum2 - sum1);
}
