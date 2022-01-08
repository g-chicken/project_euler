fn main() {
    let abandant_num: Vec<u32> = get_abandant();
    let min_sum_abandant: u32 = abandant_num[0] + abandant_num[0];

    let mut sum: u32 = 0;

    for n in 1..28124 {
        if n >= 1000 && n % 1000 == 0 {
            println!("{}", n);
        }

        if n < min_sum_abandant {
            sum += n;

            continue;
        }

        if !is_sum_of_abandants(&abandant_num, n) {
            sum += n;
        }
    }

    println!("sum: {}", sum);
}

fn get_abandant() -> Vec<u32> {
    let mut result: Vec<u32> = Vec::<u32>::new();

    for n in 1..28124 {
        if d(n) > n * 2 {
            result.push(n);
        }
    }

    result.sort();

    result
}

fn d(n: u32) -> u32 {
    let mut sum: u32 = 0;
    let mut left: u32 = 1;
    let mut right: u32 = n;

    while left <= right {
        if n % left == 0 {
            right = n / left;

            if left <= right {
                sum += left;

                if left != right {
                    sum += right;
                }
            }
        }

        left += 1;
    }

    sum
}

fn is_sum_of_abandants(abandant_num: &Vec<u32>, n: u32) -> bool {
    for a in abandant_num {
        if n <= *a {
            break;
        }

        let sub: u32 = n - *a;

        for b in abandant_num {
            if sub < *b {
                break;
            }

            if sub % *b == 0 {
                return true;
            }
        }
    }

    false
}
