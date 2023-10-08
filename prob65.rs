const CONVERGENTS: usize = 100;

fn main() {
    if CONVERGENTS == 0 {
        println!("{}", 0);
    } else if CONVERGENTS == 1 {
        println!("{}", 2);
    }

    let mut sequence = Vec::<u64>::with_capacity(CONVERGENTS);
    let mut k: u64 = 1;

    for i in 0..CONVERGENTS as usize {
        if i == 0 {
            sequence.push(2);
        } else if (i - 1) % 3 == 0 || (i - 1) % 3 == 2 {
            sequence.push(1);
        } else {
            sequence.push(k * 2);
            k += 1;
        }
    }

    let mut denominator: Vec<u8> = vec![1];
    let mut numerator: Vec<u8> = to_vec(sequence[CONVERGENTS - 1]);
    let mut i = CONVERGENTS as u64 - 2;

    loop {
        let seq_v = to_vec(sequence[i as usize]);
        let multiplication = multiply(numerator.clone(), seq_v);
        denominator = add(denominator, multiplication);
        let tmp = denominator;
        denominator = numerator;
        numerator = tmp;

        if i == 0 {
            break;
        }

        i -= 1;
    }

    let mut sum: u64 = 0;
    for v in numerator.iter() {
        sum += *v as u64;
    }

    println!(
        "{} / {} -> {}",
        show_vec(numerator),
        show_vec(denominator),
        sum
    );
}

fn to_vec(n: u64) -> Vec<u8> {
    let mut v = Vec::<u8>::with_capacity(((n as f64).log10() as usize) + 1);
    let mut nn = n;

    while nn > 0 {
        v.push((nn % 10) as u8);
        nn /= 10;
    }

    v
}

fn show_vec(v: Vec<u8>) -> String {
    let mut s: String = "".to_string();

    for n in v.iter().rev() {
        s += &n.to_string();
    }

    s
}

fn add(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut c = Vec::<u8>::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
    let mut i: usize = 0;
    let mut over: u8 = 0;

    while i < a.len() || i < b.len() {
        if i < a.len() && i < b.len() {
            let n = a[i] + b[i] + over;
            c.push(n % 10);
            over = n / 10;
        } else if a.len() > b.len() {
            c.push(a[i] + over);
            over = 0;
        } else {
            c.push(b[i] + over);
            over = 0;
        }

        i += 1;
    }

    if over > 0 {
        c.push(over);
    }

    c
}

fn multiply(a: Vec<u8>, b: Vec<u8>) -> Vec<u8> {
    let mut c = Vec::<u8>::with_capacity(
        ((a.len() as f64).log10() + (b.len() as f64).log10()).ceil() as usize,
    );

    let mut i: usize = 0;
    while i < b.len() {
        let mut d = Vec::<u8>::with_capacity(a.len() + i + 1);

        for _j in 0..i {
            d.push(0);
        }

        let bv = b[i];
        let mut over = 0;
        for av in a.iter() {
            let v = av * bv + over;
            d.push(v % 10);
            over = v / 10;
        }

        if over > 0 {
            d.push(over);
        }

        c = add(c, d);
        i += 1;
    }

    c
}
