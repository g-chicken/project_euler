fn main() {
    let mut counter: u32 = 0;

    for n in 1..101 {
        for r in 1..(n + 1) {
            if over_million_combinatorics(n as u64, r as u64) {
                counter += 1;
            }
        }
    }

    println!("{}", counter);
}

fn over_million_combinatorics(n: u64, r: u64) -> bool {
    if n < r {
        return false;
    }

    let mut c: u64 = 1;
    let mut d: Vec<u64> = Vec::<u64>::new();

    for i in (r + 1)..(n + 1) {
        c *= i;

        for j in 2..(n - r + 1) {
            if !d.contains(&j) && c % j == 0 {
                c /= j;
                d.push(j);
            }
        }

        if c >= 1000000 {
            return true;
        }
    }

    false
}

#[test]
fn test_combinatorics() {
    assert!(!over_million_combinatorics(5, 3));
    assert!(over_million_combinatorics(30, 10));
    assert!(over_million_combinatorics(100, 50));
}
