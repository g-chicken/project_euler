fn main() {
    let mut sum: u32 = 2;
    let mut a1: u32 = 1;
    let mut a2: u32 = 2;
    let mut a3: u32 = fibonacci(a1, a2);

    while a3 < 4000000 {
        if a3 % 2 == 0 {
            sum += a3;
        }

        a1 = a2;
        a2 = a3;
        a3 = fibonacci(a1, a2);
    }

    println!("sum: {}", sum);
}

fn fibonacci(a1: u32, a2: u32) -> u32 {
    return a1 + a2;
}
