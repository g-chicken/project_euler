const D: u64 = 1000;

fn main() {
    let mut d: u64 = 2;
    let mut max_d: u64 = 0;
    let mut max_x: String = "".to_string();
    let one = vec![1];

    while d <= D {
        let d_vec = to_vec(d);

        if is_square_number(&d_vec) {
            d += 1;
            continue;
        }

        let mut y: Vec<u8> = vec![1];
        loop {
            let square_y = multiply(&y, &y);
            let square_x = add(&multiply(&square_y, &d_vec), &one);

            if is_square_number(&square_x) {
                let string_square_x = show_vec(&square_x);
                println!("{} : {} - {}*{}^2 = 1", d, string_square_x, d, show_vec(&y));

                if max_x < string_square_x {
                    max_d = d;
                    max_x = string_square_x;
                }
                break;
            }

            y = add(&y, &one);
        }

        d += 1;
    }

    println!("{} ({})", max_d, max_x);
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

fn show_vec(v: &Vec<u8>) -> String {
    let mut s: String = "".to_string();

    for n in v.iter().rev() {
        s += &n.to_string();
    }

    s
}

fn compare_vec(v: &Vec<u8>, w: &Vec<u8>) -> i32 {
    if v.len() > w.len() {
        return 1;
    } else if v.len() < w.len() {
        return -1;
    }

    let mut i = v.len() - 1;
    loop {
        if v[i] > w[i] {
            return 1;
        } else if v[i] < w[i] {
            return -1;
        }

        if i == 0 {
            break;
        }

        i -= 1;
    }

    return 0;
}

fn is_square_number(v: &Vec<u8>) -> bool {
    let mut start_v = to_vec(2);
    let mut square = multiply(&start_v, &start_v);
    let one = vec![1];

    while compare_vec(&square, &v) < 0 {
        //println!(
        //    "{} vs {} = {} ({})",
        //    show_vec(&square),
        //    show_vec(&v),
        //    compare_vec(&square, &v),
        //    show_vec(&start_v)
        //);
        //println!(
        //    "{:?} vs {:?} = {} ({:?})",
        //    square,
        //    v,
        //    compare_vec(&square, &v),
        //    start_v
        //);
        start_v = add(&start_v, &one);
        square = multiply(&start_v, &start_v);
    }

    compare_vec(&square, &v) == 0
}

fn add(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut c = Vec::<u8>::with_capacity(std::cmp::max(a.len(), b.len()) + 1);
    let mut i: usize = 0;
    let mut over: u8 = 0;

    while i < a.len() || i < b.len() {
        if i < a.len() && i < b.len() {
            let n = a[i] + b[i] + over;
            c.push(n % 10);
            over = n / 10;
        } else if a.len() > b.len() {
            c.push((a[i] + over) % 10);
            over = (a[i] + over) / 10;
        } else {
            c.push((b[i] + over) % 10);
            over = (b[i] + over) / 10;
        }

        i += 1;
    }

    if over > 0 {
        while over > 0 {
            c.push(over % 10);
            over /= 10;
        }
    }

    c
}

fn multiply(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
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
            while over > 0 {
                d.push(over % 10);
                over /= 10;
            }
        }

        c = add(&c, &d);
        i += 1;
    }

    c
}
