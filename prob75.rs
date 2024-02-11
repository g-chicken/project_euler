use std::collections::HashMap;
const LIMIT_L: u32 = 1_500_000;

fn main() {
    let triangle_set = get_right_angle_triangle(LIMIT_L);
    let mut length_map = HashMap::<u32, u32>::new();

    for triangle in triangle_set.iter() {
        let mut m = 1;
        let mut l = triangle
            .iter()
            .map(|v| v * m)
            .fold(0, |accumulator, v| accumulator + v);

        while l <= LIMIT_L {
            let count = length_map.get(&l);
            if count == None {
                length_map.insert(l, 1);
            } else {
                length_map.insert(l, count.unwrap() + 1);
            }

            m += 1;
            l = triangle
                .iter()
                .map(|v| v * m)
                .fold(0, |accumulator, v| accumulator + v);
        }
    }

    //println!("{:?}", length_map);

    let mut result = 0;
    for &count in length_map.values() {
        if count == 1 {
            result += 1;
        }
    }

    println!("{}", result);
}

fn get_right_angle_triangle(p: u32) -> Vec<Vec<u32>> {
    let mut m: u32 = 1;
    let mut rat: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();

    loop {
        let mut is_more_p = true;

        let mut n: u32 = 2;
        let mut a: u32 = ((m * m) as i64 - (n * n) as i64).abs() as u32;
        let mut b: u32 = 2 * m * n;
        let mut c: u32 = m * m + n * n;

        println!("{}, {} ({}, {}, {})", m, n, a, b, c);

        while a + b + c <= p {
            is_more_p = false;

            let mut gcd: u32 = euclidean_algo(a, b);
            gcd = euclidean_algo(gcd, c);

            if gcd > 1 {
                a /= gcd;
                b /= gcd;
                c /= gcd;
            }

            let mut v: Vec<u32> = vec![a, b, c];
            v.sort();

            if !rat.contains(&v) {
                rat.push(v);
            }

            n += 2;
            a = ((m * m) as i64 - (n * n) as i64).abs() as u32;
            b = 2 * m * n;
            c = m * m + n * n;
        }

        if is_more_p {
            break;
        }

        m += 2;
    }

    rat
}

fn euclidean_algo(a: u32, b: u32) -> u32 {
    let mut n: u32 = b;
    let mut r: u32 = a % b;

    while r > 0 {
        let m: u32 = n;
        n = r;
        r = m % n;
    }

    n
}
