fn main() {
    let mut finish = false;

    // a + b + c = 1000
    // a < b < c
    // => 2a < b + c = 1000 - a <=>  a < 334
    for a in 1..334 {
        let mut b = a + 1;

        // a + b + c = 1000
        // a < b < c
        // => a + 2 * b < a + b + c = 1000
        while a + 2 * b < 1000 {
            let c = 1000 - a - b;

            if a*a + b*b == c*c {
                println!("{} (a = {}, b = {}, c = {})", a*b*c, a, b, c);
                finish = true;

                break;
            }

            b += 1;
        }

        if finish {
            break;
        }
    }
}
