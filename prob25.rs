fn main() {
    let mut f: [Vec<u8>; 3] = [vec![1], vec![1], vec![0]];
    let mut i: u32 = 1;

    loop {
        i += 1;

        let index1: usize = (i % 3) as usize;
        let index2: usize = ((i - 1) % 3) as usize;
        let index3: usize = ((i - 2) % 3) as usize;

        f[index1] = add(&f[index2], &f[index3]);

        if f[index1].len() >= 1000 {
            break;
        }
    }

    println!("{} ({:?})", i + 1, f[(i % 3) as usize]);
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
