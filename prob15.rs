fn main() {
    let mut d: u128 = 1;
    let mut num: u128 = 1;

    for n in 1..21 {
        num *= n + 20;
        d *= n;
    }

    println!("{}", num / d);
}
