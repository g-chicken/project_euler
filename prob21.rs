fn main() {
    let mut sum: u32 = 0;

    for n in 1..10000 {
        let a = d(n);

        if d(a) == n && n < a {
            sum += n + a;
        }
    }

    println!("sum: {}", sum);
}

fn d(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut left: u32 = 1;
    let mut right: u32 = n;

    while left <= right {
        if n % left == 0 {
            right = n / left;

            if left < right {
                sum += left;

                if right != n {
                    sum += right;
                }
            }
        }

        left += 1;
    }

    sum
}
