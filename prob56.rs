fn main() {
    let mut max: u32 = 0;

    for a in 1..100 {
        for b in 1..100 {
            let vec: Vec<u8> = power(&to_vec(a), b as u16);
            let mut n: u32 = 0;

            for v in &vec {
                n += *v as u32;
            }

            if max < n {
                max = n;
            }
        }
    }

    println!("{}", max);
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

fn power(a: &Vec<u8>, exp: u16) -> Vec<u8> {
    let n: u64 = to_num(a);
    let mut result: Vec<u8> = vec![1];

    for _i in 0..exp {
        let r: Vec<u8> = multi(&result, n);
        result.clear();
        result.extend(r.iter().copied());
    }

    result
}

fn multi(a: &Vec<u8>, b: u64) -> Vec<u8> {
    let mut result: Vec<u8> = vec![0];

    for _i in 0..b {
        let r: Vec<u8> = add(&result, a);
        result.clear();
        result.extend(r.iter().copied());
    }

    result
}

fn to_num(vec: &Vec<u8>) -> u64 {
    let mut num: u64 = 0;

    for v in vec.iter().rev() {
        num = num * 10 + (*v as u64);
    }

    num
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
fn test_add() {
    assert_eq!(vec![1], add(&vec![0], &vec![1]));
    assert_eq!(vec![2], add(&vec![1], &vec![1]));
    assert_eq!(vec![5, 1], add(&vec![8], &vec![7]));
    assert_eq!(vec![5, 1], add(&vec![2, 1], &vec![3]));
    assert_eq!(vec![3, 2, 1], add(&vec![0, 2, 1], &vec![3]));
    assert_eq!(vec![3, 2, 1], add(&vec![4, 7], &vec![9, 4]));
}

#[test]
fn test_power() {
    assert_eq!(vec![1], power(&vec![1], 100));
    assert_eq!(vec![6, 1], power(&vec![2], 4));
    assert_eq!(vec![4, 4, 1], power(&vec![2, 1], 2));
}

#[test]
fn test_muitl() {
    assert_eq!(vec![0], multi(&vec![0], 100));
    assert_eq!(vec![0, 0, 1], multi(&vec![1], 100));
    assert_eq!(vec![2, 3], multi(&vec![2], 16));
    assert_eq!(vec![4, 8], multi(&vec![1, 2], 4));
    assert_eq!(vec![4, 4, 1], multi(&vec![2, 1], 12));
}

#[test]
fn test_to_num() {
    assert_eq!(0, to_num(&vec![0]));
    assert_eq!(123, to_num(&vec![3, 2, 1]));
    assert_eq!(1204269102, to_num(&vec![2, 0, 1, 9, 6, 2, 4, 0, 2, 1]));
}

#[test]
fn test_to_vec() {
    assert_eq!(vec![0], to_vec(0));
    assert_eq!(vec![3, 2, 1], to_vec(123));
    assert_eq!(vec![2, 0, 1, 9, 6, 2, 4, 0, 2, 1], to_vec(1204269102));
}
