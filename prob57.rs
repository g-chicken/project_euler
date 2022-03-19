fn main() {
    let mut n: Vec<u8> = vec![1];
    let mut d: Vec<u8> = vec![2];
    let mut count = 0;

    for i in 0..1000 {
        if i > 0 {
            let mut tmp_d: Vec<u8> = Vec::<u8>::new();
            tmp_d.extend(d.iter().copied());

            (n, d) = (d, add(&multi(&tmp_d, 2), &n));
        }

        let result_n: Vec<u8> = add(&n, &d);

        if result_n.len() > d.len() {
            count += 1;
        }
    }

    println!("{}", count);
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
