fn main() {
    let mut num: u32 = 1;
    let mut order: u32 = 1;
    let mut product: u32 = 1;
    let mut get_order: u32 = 1;

    while order < 1000000 {
        let digit_len: u32 = get_digit_len(num);

        if order + digit_len - 1 >= get_order {
            let n: u32 = match get_digit(num, order + digit_len - get_order) {
                Some(d) => d,
                None => panic!(
                    "invalid! (num: {}, term: {})",
                    num,
                    order + digit_len - get_order + 1
                ),
            };

            product *= n;
            get_order *= 10;
        }

        order += digit_len;
        num += 1;
    }

    println!("{}", product);
}

fn get_digit_len(num: u32) -> u32 {
    if num == 0 {
        return 1;
    }

    let mut n: u32 = num;
    let mut len: u32 = 0;

    while n > 0 {
        len += 1;
        n /= 10;
    }

    len
}

fn get_digit(num: u32, digit_term_from_right: u32) -> Option<u32> {
    if digit_term_from_right == 0 {
        return None;
    }

    if digit_term_from_right > get_digit_len(num) {
        return None;
    }

    let mut n: u32 = num;
    let mut r: u32 = 0;
    let mut d: u32 = digit_term_from_right;

    while d > 0 {
        r = n % 10;
        n /= 10;
        d -= 1;
    }

    Some(r)
}

#[test]
fn test_get_digit_len() {
    assert_eq!(1, get_digit_len(3), "expected 1 but not (3)");
    assert_eq!(1, get_digit_len(0), "expected 1 but not (0)");
    assert_eq!(4, get_digit_len(1234), "expected 4 but not (1234)");
}

#[test]
fn test_get_digit() {
    assert_eq!(Some(3), get_digit(3, 1), "expected 3 but not (3)");
    assert_eq!(Some(4), get_digit(1234, 1), "expected 4 but not (1234)");
    assert_eq!(Some(3), get_digit(1234, 2), "expected 3 but not (1234)");
    assert_eq!(Some(2), get_digit(1234, 3), "expected 2 but not (1234)");
    assert_eq!(Some(1), get_digit(1234, 4), "expected 1 but not (1234)");

    assert_eq!(None, get_digit(1234, 0), "expected None but not");
    assert_eq!(None, get_digit(1234, 5), "expected None but not");
}
