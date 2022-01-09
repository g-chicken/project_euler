// 9! * 8 = 2,903,040, so #digits of upper number is 7.
fn main() {
    let mut sum: u32 = 0;

    for n in 3..10000000 {
        if n == sum_factorial(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn sum_factorial(num: u32) -> u32 {
    let mut n: u32 = num;
    let mut sum: u32 = 0;

    while n > 0 {
        sum += factorial(n % 10);
        n /= 10;
    }

    sum
}

fn factorial(num: u32) -> u32 {
    let mut f: u32 = 1;

    for n in 1..(num + 1) {
        f *= n;
    }

    f
}
