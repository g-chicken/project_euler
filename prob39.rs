fn main() {
    let mut counter: [u32; 1001] = [0; 1001];
    let rats: Vec<Vec<u32>> = get_right_angle_triangle(1000);

    for rat in &rats {
        let sum: u32 = rat.iter().sum();
        counter[sum as usize] += 1;

        let mut i: u32 = 2;
        while sum * i <= 1000 {
            counter[(sum * i) as usize] += 1;
            i += 1;
        }
    }

    let solutions: &u32 = counter.iter().max().unwrap();
    println!(
        "{} (solutions: {})",
        counter.iter().position(|&f| f == *solutions).unwrap(),
        solutions,
    );
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

#[test]
fn test_get_right_angle_triangle() {
    assert_eq!(Vec::<Vec<u32>>::new(), get_right_angle_triangle(0), "p = 0");
    assert_eq!(Vec::<Vec<u32>>::new(), get_right_angle_triangle(1), "p = 1");
    assert_eq!(
        Vec::<Vec<u32>>::new(),
        get_right_angle_triangle(10),
        "p = 10"
    );
    assert_eq!(
        Vec::<Vec<u32>>::new(),
        get_right_angle_triangle(11),
        "p = 11"
    );

    assert_eq!(vec![vec![3, 4, 5]], get_right_angle_triangle(12), "p = 12");
    assert_eq!(vec![vec![3, 4, 5]], get_right_angle_triangle(13), "p = 13");
    assert_eq!(vec![vec![3, 4, 5]], get_right_angle_triangle(14), "p = 14");
    assert_eq!(vec![vec![3, 4, 5]], get_right_angle_triangle(15), "p = 15");
    assert_eq!(vec![vec![3, 4, 5]], get_right_angle_triangle(29), "p = 29");

    assert_eq!(
        vec![vec![3, 4, 5], vec![5, 12, 13]],
        get_right_angle_triangle(30),
        "p = 30"
    );
    assert_eq!(
        vec![vec![3, 4, 5], vec![5, 12, 13]],
        get_right_angle_triangle(39),
        "p = 39"
    );

    assert_eq!(
        vec![vec![3, 4, 5], vec![8, 15, 17], vec![5, 12, 13]],
        get_right_angle_triangle(40),
        "p = 40"
    );
    assert_eq!(
        vec![vec![3, 4, 5], vec![8, 15, 17], vec![5, 12, 13]],
        get_right_angle_triangle(55),
        "p = 55"
    );

    assert_eq!(
        vec![
            vec![3, 4, 5],
            vec![8, 15, 17],
            vec![5, 12, 13],
            vec![7, 24, 25]
        ],
        get_right_angle_triangle(56),
        "p = 56"
    );
}

#[test]
fn test_euclidean_algo() {
    assert_eq!(1, euclidean_algo(1, 1), "expected 1 but not (1)");
    assert_eq!(1, euclidean_algo(97, 137), "expected 1 but not (1)");
    assert_eq!(1, euclidean_algo(137, 97), "expected 1 but not (1)");
    assert_eq!(36, euclidean_algo(144, 180), "expected 1 but not (36)");
    assert_eq!(36, euclidean_algo(180, 144), "expected 1 but not (36)");
}
