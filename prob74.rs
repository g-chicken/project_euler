use std::collections::HashMap;

fn main() {
    let mut sixty_chanins = 0;
    let chain = HashMap::<u128, u128>::from([
        (0, 2),
        (1, 1),
        (2, 1),
        (145, 1),
        (40585, 1),
        (169, 3),
        (363601, 3),
        (1454, 3),
        (871, 2),
        (45361, 2),
        (872, 2),
        (45362, 2),
    ]);

    for n in 2..1_000_001 {
        //for n in 14558..14560 {
        //println!("{}", n);
        let mut count = 0;
        let mut known_chain = chain.get(&n);
        if known_chain != None {
            continue;
        }

        let mut a = n;
        while known_chain == None {
            //println!("   {}", a);
            count += 1;
            a = add_digit_factorial(a);
            known_chain = chain.get(&a);
        }

        count += known_chain.unwrap();
        if count == 60 {
            println!("chain 60: {}", n);
            sixty_chanins += 1;
        } else if count > 60 {
            println!("[ERR] over chain 60: {}", n);
        }
    }

    println!("{}", sixty_chanins);
}

fn add_digit_factorial(n: u128) -> u128 {
    let mut a = n;
    let mut result: u128 = 0;

    while a > 0 {
        result += get_factorial(a % 10);
        a /= 10;
    }

    result
}

fn get_factorial(n: u128) -> u128 {
    if n < 2 {
        return 1;
    }

    let mut f: u128 = 1;

    for a in 2..(n + 1) {
        f *= a;
    }

    f
}
