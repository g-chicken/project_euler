const LIMIT_N: i32 = 10_000;

fn main() {
    let mut odd_count = 0;
    let mut n: i32 = 2;

    while n <= LIMIT_N {
        let mut integer_parts = Vec::<u16>::new();

        let n_sqrt = (n as f64).sqrt();

        if n_sqrt.floor() == n_sqrt.ceil() {
            n += 1;
            continue;
        }

        let first_integer_part = n_sqrt.floor() as u16;
        let first_denominator_integer_part = -1 * first_integer_part as i32;

        let mut denominator_integer_part = first_denominator_integer_part;
        let mut numerator: i32 = 1;

        //println!("{} , {}", denominator_integer_part, numerator);

        loop {
            let mut coefficient = numerator;
            numerator = n - denominator_integer_part * denominator_integer_part;
            denominator_integer_part = -denominator_integer_part;

            (coefficient, numerator) = reduce(coefficient, numerator);

            if coefficient > 1 {
                println!("coefficient is more than 1. ({})", n);
            }

            let integer_part =
                ((n_sqrt + denominator_integer_part as f64) / numerator as f64).floor() as i32;

            integer_parts.push(integer_part as u16);
            denominator_integer_part -= integer_part as i32 * numerator;

            //println!("{} , {}", denominator_integer_part, numerator);

            if numerator == 1 && denominator_integer_part == first_denominator_integer_part {
                break;
            }
        }

        //println!("{} = [{}; ({:?})]", n, first_integer_part, integer_parts);

        if integer_parts.len() % 2 == 1 {
            odd_count += 1;
        }

        n += 1;
    }

    println!("{}", odd_count);
}

fn reduce(denominator: i32, numerator: i32) -> (i32, i32) {
    if denominator == 1 || numerator == 1 {
        return (denominator, numerator);
    }

    let gcd = get_gcd(denominator, numerator);

    (denominator / gcd, numerator / gcd)
}

fn get_gcd(m: i32, n: i32) -> i32 {
    let mut a = if m > n { m } else { n };
    let mut b = if m > n { n } else { m };
    let mut r = a % b;

    while r > 0 {
        a = b;
        b = r;
        r = a % b;
    }

    b
}
