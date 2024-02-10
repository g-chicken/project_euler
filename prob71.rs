const LIMIT: u32 = 1_000_000;
const TARGET_NUMERATOR: u32 = 3;
const TARGET_DENOMINATOR: u32 = 7;

fn main() {
    let mut numerator = 0;
    let mut denominator = 0;
    let mut d = LIMIT;

    while d > 1 {
        let (n, ok) = get_numerator_by_binary_search(d);
        //println!("{} / {} ({})", n, d, ok);
        if ok {
            if numerator == 0 {
                numerator = n;
                denominator = d;
            } else {
                let gcd = get_gcd(d, denominator);
                if (n as u128) * ((denominator / gcd) as u128)
                    > (numerator as u128) * ((d / gcd) as u128)
                {
                    numerator = n;
                    denominator = d;
                }
            }
        }

        d -= 1;
    }

    println!("{} / {}", numerator, denominator);
}

fn get_numerator_by_binary_search(n: u32) -> (u32, bool) {
    let gcd = get_gcd(n, TARGET_DENOMINATOR);
    let mut left = 1;
    let mut right = n - 1;

    while right - left > 1 {
        let middle = (left + right) / 2;

        if middle * (TARGET_DENOMINATOR / gcd) < TARGET_NUMERATOR * (n / gcd) {
            left = middle;
        } else if middle * (TARGET_DENOMINATOR / gcd) > TARGET_NUMERATOR * (n / gcd) {
            right = middle;
        } else {
            return (middle - 1, middle > 0);
        }
    }

    let mut numerator = left;
    if right < left {
        numerator = right;
    }

    if numerator * (TARGET_DENOMINATOR / gcd) > TARGET_NUMERATOR * (n / gcd) {
        numerator -= 1;
    }

    (numerator, numerator > 0)
}

fn get_gcd(a: u32, b: u32) -> u32 {
    let mut n = b;
    let mut r = a % b;

    while r > 0 {
        let m = n;
        n = r;
        r = m % n;
    }

    n
}
