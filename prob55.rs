fn main() {
    let mut count: u32 = 0;

    for n in 1..10000 {
        let mut v: Vec<u8> = to_vec(n);
        let mut r: Vec<u8> = palindrome(&v);
        let mut loop_count: u8 = 0;

        while loop_count < 50 {
            let a: Vec<u8> = add(&v, &r);
            r = palindrome(&a);

            if a == r {
                break;
            }

            v = a;
            loop_count += 1;
        }

        if loop_count == 50 {
            count += 1;
        }
    }

    println!("{}", count)
}

fn palindrome(a: &Vec<u8>) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::<u8>::new();
    let mut i: usize = a.len() - 1;

    loop {
        result.push(a[i]);

        if i == 0 {
            break;
        }

        i -= 1;
    }

    result
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut over: u8 = 0;
    let mut i: usize = 0;
    let mut result: Vec<u8> = Vec::<u8>::new();

    while i < a.len() || i < b.len() {
        let n: u8;

        if i < a.len() && i < b.len() {
            n = a[i] + b[i] + over;
        } else if i < a.len() {
            n = a[i] + over;
        } else {
            n = b[i] + over;
        }

        result.push(n % 10);
        over = n / 10;

        i += 1;
    }

    while over > 0 {
        result.push(over % 10);
        over /= 10;
    }

    result
}

fn to_vec(num: u64) -> Vec<u8> {
    if num == 0 {
        return vec![0];
    }

    let mut n: u64 = num;
    let mut result: Vec<u8> = Vec::<u8>::new();

    while n > 0 {
        result.push((n % 10) as u8);
        n /= 10;
    }

    result
}

#[test]
fn test_palindrome() {
    assert_eq!(vec![0], palindrome(&vec![0]));
    assert_eq!(vec![1], palindrome(&vec![1]));
    assert_eq!(vec![1, 5], palindrome(&vec![5, 1]));
    assert_eq!(vec![1, 2, 3], palindrome(&vec![3, 2, 1]));
}

#[test]
fn test_add() {
    assert_eq!(vec![1], add(&vec![0], &vec![1]));
    assert_eq!(vec![2], add(&vec![1], &vec![1]));
    assert_eq!(vec![5, 1], add(&vec![8], &vec![7]));
    assert_eq!(vec![5, 1], add(&vec![2, 1], &vec![3]));
    assert_eq!(vec![3, 2, 1], add(&vec![0, 2, 1], &vec![3]));
    assert_eq!(vec![3, 2, 1], add(&vec![4, 7], &vec![9, 4]));
}

#[test]
fn test_to_vec() {
    assert_eq!(vec![0], to_vec(0));
    assert_eq!(vec![3, 2, 1], to_vec(123));
    assert_eq!(vec![2, 0, 1, 9, 6, 2, 4, 0, 2, 1], to_vec(1204269102));
}
