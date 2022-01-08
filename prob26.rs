use std::convert::TryInto;

fn main() {
    let mut max: u32 = 0;
    let mut result: u32 = 0;

    for n in 2..1000 {
        let cycle_len: u32 = d(n);

        if cycle_len > max {
            max = cycle_len;
            result = n;
        }
    }

    println!("{} (cycle length: {})", result, max);
}

fn d(n: u32) -> u32 {
    let mut v: Vec<u32> = Vec::<u32>::new();
    let mut r: u32 = 1;

    for i in 0..((n - 1) * 2) as usize {
        let r10: u32 = r * 10;

        if v.contains(&r10) {
            return (i - v.iter().position(|&f| f == r10).unwrap())
                .try_into()
                .unwrap();
        }

        v.push(r10);

        if r10 < n {
            r = r10;

            continue;
        }

        if r10 % n == 0 {
            break;
        }

        r = r10 % n;
    }

    0
}
