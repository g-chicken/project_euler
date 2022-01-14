fn main() {
    let mut sum: u64 = 0;

    for n in 1..1001 {
        let mut p: u64 = 1;
        for _i in 0..n {
            p = p * n % 10000000000;
        }

        sum += p % 10000000000;
    }

    println!("{}", sum % 10000000000);
}
