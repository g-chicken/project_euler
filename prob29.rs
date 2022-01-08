fn main() {
    let mut bases: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();

    for i in 0..11 {
        if i <= 1 {
            bases.push(Vec::<u32>::new());

            continue;
        }

        let mut v: Vec<u32> = Vec::<u32>::new();
        let mut n: u32 = 1;

        while (i as u32).pow(n) <= 100 {
            v.push((i as u32).pow(n));
            n += 1;
        }

        bases.push(v);
    }

    let mut vec: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
    for _i in 0..101 {
        vec.push(Vec::<u32>::new());
    }

    let mut terms: u32 = 0;
    for a in 2..101 {
        let mut base: usize = a;
        let mut index: u32 = 1;

        for i in 2..bases.len() {
            match bases[i].iter().position(|&f| f == a as u32) {
                None => { /* nothing to do */ }
                Some(num) => {
                    base = i;
                    index = (num + 1) as u32;
                    break;
                }
            }
        }

        let mut b: u32 = 2 * index;
        while b <= 100 * index {
            if !vec[base].contains(&b) {
                terms += 1;
                vec[base].push(b);
            }

            b += index;
        }
    }

    println!("{}", terms);
}
