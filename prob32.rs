fn main() {
    let mut sum: u32 = 0;

    for n in 1..100000 {
        if d(n) {
            sum += n;
        }
    }

    println!("{}", sum);
}

fn d(n: u32) -> bool {
    let mut left: u32 = 1;
    let mut right: u32 = n;

    while left <= right {
        if n % left == 0 {
            right = n / left;

            if is_pandigital(left, right, n) {
                return true;
            }
        }

        left += 1;
    }

    false
}

fn is_pandigital(a: u32, b: u32, c: u32) -> bool {
    let mut v: Vec<u32> = Vec::<u32>::new();

    let mut a_r: u32 = a;
    while a_r > 0 {
        v.push(a_r % 10);
        a_r /= 10;
    }

    let mut b_r: u32 = b;
    while b_r > 0 {
        v.push(b_r % 10);
        b_r /= 10;
    }

    let mut c_r: u32 = c;
    while c_r > 0 {
        v.push(c_r % 10);
        c_r /= 10;
    }

    if v.len() != 9 {
        return false;
    }

    for i in 1..10 {
        if !v.contains(&i) {
            return false;
        }
    }

    true
}
