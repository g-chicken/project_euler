fn main() {
    let mut result = 0;
    let mut max: u32 = 0;

    for n in 1..1000000 {
        let mut term: u32 = 1;
        let mut num: u64 = n as u64;

        while num != 1 {
            num = collatz(num);
            term += 1;
        }

        if term > max { 
            result = n;
            max = term;
        }
    }

    println!("{} (term: {})", result, max);
}

fn collatz(n: u64) -> u64 {
    if n % 2 == 0 {
        return n / 2;
    }

    n * 3 + 1
}
