fn main() {
    let mut count: u32 = 1; // 1^1 = 1
    let mut base: f64 = 2.0;

    while base.log10() < 1.0 {
        let mut n: f64 = 1.0;

        while n * base.log10() >= n - 1.0 {
            count += 1;
            n += 1.0;
        }

        base += 1.0;
    }

    println!("{}", count);
}
