fn main() {
    let mut n: u64 = 3;
    let mut primes: Vec<u64> = Vec::<u64>::new();
    let mut finish: bool = false;

    while !finish {
        if !create_and_check_primes(n, &mut primes) {
            n += 2;

            continue;
        }

        let families: Vec<Vec<u64>> = get_family(n);

        for family in &families {
            let mut primes_in_family: Vec<u64> = Vec::<u64>::new();

            for f in family {
                if is_prime(*f, &primes) {
                    primes_in_family.push(*f);
                }
            }

            if primes_in_family.len() > 7 {
                println!("{:?}", primes_in_family);
                finish = true;

                break;
            }

            if finish {
                break;
            }
        }

        n += 2;
    }
}

fn get_family(num: u64) -> Vec<Vec<u64>> {
    let mut n: u64 = num;
    let mut digits: Vec<u64> = Vec::<u64>::new();
    let mut digits_num: [u64; 10] = [0; 10];

    while n > 0 {
        let d: u64 = n % 10;
        digits.insert(0, d);
        digits_num[d as usize] += 1;
        n /= 10;
    }

    let mut result: Vec<Vec<u64>> = Vec::new();

    for i in 0..10 {
        if digits_num[i] < 2 {
            continue;
        }

        let mut numbers: Vec<u64> = Vec::<u64>::new();

        for j in 0..10 {
            let mut generated_num: u64 = 0;
            for d in &digits {
                if *d == i as u64 {
                    generated_num = generated_num * 10 + j as u64;
                } else {
                    generated_num = generated_num * 10 + *d;
                }
            }

            if generated_num > 10u64.pow(digits.len() as u32 - 1) - 1 {
                numbers.push(generated_num);
            }
        }

        result.push(numbers);
    }

    result
}

fn create_and_check_primes(num: u64, primes: &mut Vec<u64>) -> bool {
    if primes.len() == 0 {
        primes.push(2);
    }

    let num_sqrt: f64 = (num as f64).sqrt();
    let mut last: u64 = *primes.last().unwrap();

    while *primes.last().unwrap() as f64 <= num_sqrt {
        last += 1;

        if is_prime(last, primes) {
            primes.push(last);
        }
    }

    is_prime(num, primes)
}

fn is_prime(num: u64, primes: &Vec<u64>) -> bool {
    let num_sqrt: f64 = (num as f64).sqrt();
    let mut i: usize = 0;

    while i < primes.len() && primes[i] as f64 <= num_sqrt {
        if num % primes[i] == 0 {
            return false;
        }

        i += 1;
    }

    true
}

#[test]
fn test_get_family() {
    assert_eq!(get_family(1), Vec::<Vec<u64>>::new());
    assert_eq!(get_family(10), Vec::<Vec<u64>>::new());
    assert_eq!(get_family(123), Vec::<Vec<u64>>::new());
    assert_eq!(get_family(123456789), Vec::<Vec<u64>>::new());

    assert_eq!(
        get_family(11),
        vec![vec![11, 22, 33, 44, 55, 66, 77, 88, 99]]
    );
    assert_eq!(
        get_family(2132),
        vec![vec![1131, 2132, 3133, 4134, 5135, 6136, 7137, 8138, 9139]]
    );
    assert_eq!(
        get_family(1232),
        vec![vec![
            1030, 1131, 1232, 1333, 1434, 1535, 1636, 1737, 1838, 1939
        ]]
    );
    assert_eq!(
        get_family(12323),
        vec![
            vec![10303, 11313, 12323, 13333, 14343, 15353, 16363, 17373, 18383, 19393],
            vec![12020, 12121, 12222, 12323, 12424, 12525, 12626, 12727, 12828, 12929],
        ]
    );
    assert_eq!(
        get_family(213224),
        vec![vec![
            113114, 213224, 313334, 413444, 513554, 613664, 713774, 813884, 913994
        ]]
    );
}

#[test]
fn test_create_and_check_primes() {
    struct Testcase {
        input_num: u64,
        input_vec: Vec<u64>,
        ok: bool,
        want_vec: Vec<u64>,
    }

    let testcases: [Testcase; 6] = [
        Testcase {
            input_num: 2,
            input_vec: vec![],
            ok: true,
            want_vec: vec![2],
        },
        Testcase {
            input_num: 3,
            input_vec: vec![],
            ok: true,
            want_vec: vec![2],
        },
        Testcase {
            input_num: 12,
            input_vec: vec![2, 3, 5],
            ok: false,
            want_vec: vec![2, 3, 5],
        },
        Testcase {
            input_num: 23,
            input_vec: vec![2, 3, 5],
            ok: true,
            want_vec: vec![2, 3, 5],
        },
        Testcase {
            input_num: 96,
            input_vec: vec![2, 3],
            ok: false,
            want_vec: vec![2, 3, 5, 7, 11],
        },
        Testcase {
            input_num: 97,
            input_vec: vec![2, 3],
            ok: true,
            want_vec: vec![2, 3, 5, 7, 11],
        },
    ];

    for tc in testcases {
        let mut v: Vec<u64> = Vec::<u64>::new();
        v.extend(tc.input_vec.iter().copied());

        let ok: bool = create_and_check_primes(tc.input_num, &mut v);

        assert_eq!(tc.ok, ok);
        assert_eq!(tc.want_vec, v);
    }
}

#[test]
fn test_is_prime() {
    let primes: Vec<u64> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47];

    assert!(is_prime(3, &primes));
    assert!(is_prime(29, &primes));
    assert!(is_prime(53, &primes));
    assert!(is_prime(67, &primes));
    assert!(is_prime(71, &primes));
    assert!(is_prime(83, &primes));
    assert!(is_prime(97, &primes));
    assert!(is_prime(373, &primes));

    assert!(is_prime(53 * 53, &primes));

    assert!(!is_prime(4, &primes));
    assert!(!is_prime(39, &primes));
    assert!(!is_prime(57, &primes));
}
