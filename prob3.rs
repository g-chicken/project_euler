fn main() {
    let mut target: u64 = 600851475143;
    let mut num: u64 = 2;

    while num < target {
        println!("target: {}, num: {}", target, num);

        if target % num == 0 {
            target /= num;
            num = 2;

            continue;
        }

        num += 1;
    }

    println!("{}", target);
}
