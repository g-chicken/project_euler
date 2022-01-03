fn main() {
    let mut max = 0;

    for i in 100..1000 {
        for j in i..1000 {
            if is_palindromic(i * j) {
                println!("{} * {} = {}", i, j, i*j);

                if i * j > max {
                    max = i * j;
                }
            }
        }
    }

    println!("max: {}", max);
}

fn is_palindromic(num: u64) -> bool {
    let mut palindromic: u64 = 0;
    let mut n: u64 = num;

    while n > 0 {
        palindromic = palindromic * 10 + n%10;
        n /= 10;
    }

    return num == palindromic;
}
