use std::time::{SystemTime, UNIX_EPOCH};

//   0
//     \
//      5  1
//     /  \|
//    9    6
//  / |    |
// 4  8 -- 7 - 2
//    |
//    3
const INDEX: usize = 10;
const LINE_FACTS: [[usize; 3]; 5] = [[0, 5, 6], [1, 6, 7], [2, 7, 8], [3, 8, 9], [4, 9, 5]];
const PENALTY_WEIGHT: f64 = 10e_15;

fn main() {
    let mut result: [u8; INDEX] = [1; INDEX];
    let mut result_eval = -1.0;
    let mut result_line_sums = get_line_sums(result);
    let mut multi_start_count = 0;

    while result_eval <= 0.0
        || result_line_sums.first().unwrap() != result_line_sums.last().unwrap()
    {
        multi_start_count += 1;

        // initial
        let mut ring: [u8; INDEX] = create_initial();
        let mut eval = evaluate(ring);
        let mut iterate = 0;
        let mut line_sums = get_line_sums(ring);

        while iterate < 10000 && line_sums.first().unwrap() != line_sums.last().unwrap() {
            iterate += 1;
            //if iterate % 100000 == 0 {
            //    print!(
            //        "iter: {}\nsum : {:?}\neval: {}\nring: {:?}\n\n",
            //        iterate, line_sums, eval, ring
            //    );
            //}

            // strategy
            let ring_clone = strategy(ring);
            let eval_for_clone = evaluate(ring_clone);
            //println!(
            //    "{} vs {} ({:?}, {}, {})",
            //    eval, eval_for_clone, ring_clone, i0, i1
            //);
            if eval < eval_for_clone {
                eval = eval_for_clone;
                ring = ring_clone;
                line_sums = get_line_sums(ring);
            }
        }

        print!(
            "multi: {}\niter : {}\nsum  : {} ({})\neval : {}\nring : {:?}\n\n",
            multi_start_count,
            iterate,
            line_sums[0],
            line_sums.first().unwrap() == line_sums.last().unwrap(),
            eval,
            ring
        );

        if result_eval < eval {
            result_eval = eval;
            result = ring;
            result_line_sums = get_line_sums(result);
        }
    }

    print!(
        "finish!\nsum : {}\neval: {}\nring: {:?}\n",
        result_line_sums[0], result_eval, result
    );
}

fn create_initial() -> [u8; INDEX] {
    let mut ring: [u8; INDEX] = [0; INDEX];
    let mut used: [bool; INDEX + 1] = [false; INDEX + 1];
    let mut filled = 0;

    while filled < INDEX {
        let mut epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before UNIX_EPOCH")
            .as_micros();

        while epoch > 0 && filled < INDEX {
            let mut n = epoch % 10;
            if n == 0 {
                n = 10;
            }
            if !used[n as usize] {
                ring[filled] = n as u8;
                used[n as usize] = true;
                filled += 1;

                if filled == 4 && !used[10] {
                    ring[filled] = 10;
                    used[10] = true;
                    filled += 1;
                }
            }

            epoch /= 10;
        }
    }

    ring
}

fn strategy(ring: [u8; INDEX]) -> [u8; INDEX] {
    let epoch = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time is before UNIX_EPOCH")
        .as_micros();

    if epoch % 3 == 0 {
        return insert_strategy(ring);
    }

    exchange_strategy(ring)
}

fn insert_strategy(ring: [u8; INDEX]) -> [u8; INDEX] {
    let mut target: usize = 0;
    let mut move_to: usize = 0;
    while target == move_to
        || (ring[target] == 10 && move_to >= 5)
        || (ring[4] == 10 && target >= 5 && move_to < 5)
    {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before UNIX_EPOCH")
            .as_micros();
        target = (epoch % 10) as usize;
        move_to = ((epoch / 10) % 10) as usize;
    }

    let mut ring_clone: [u8; INDEX] = [0; INDEX];
    for i in 0..ring_clone.len() {
        if i == move_to {
            ring_clone[i] = ring[target];
            continue;
        }

        if target > move_to {
            if i <= target && i > move_to {
                ring_clone[i] = ring[i - 1];
            } else {
                ring_clone[i] = ring[i];
            }
        } else {
            if i >= target && i < move_to {
                ring_clone[i] = ring[i + 1];
            } else {
                ring_clone[i] = ring[i];
            }
        }
    }

    //println!("{:?}, {}, {} -> {:?}", ring, target, move_to, ring_clone);

    ring_clone
}

fn exchange_strategy(ring: [u8; INDEX]) -> [u8; INDEX] {
    let mut i0: usize = 0;
    let mut i1: usize = 0;
    while i0 == i1
        || (ring[i0] == 10 && i1 >= 5 && i1 <= 9)
        || (ring[i1] == 10 && i0 >= 5 && i0 <= 9)
    {
        let epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is before UNIX_EPOCH")
            .as_micros();
        i0 = (epoch % 10) as usize;
        i1 = ((epoch / 10) % 10) as usize;
    }

    let mut ring_clone = ring.clone();
    let tmp = ring_clone[i0];
    ring_clone[i0] = ring_clone[i1];
    ring_clone[i1] = tmp;

    ring_clone
}

fn evaluate(ring: [u8; INDEX]) -> f64 {
    let line_nums: Vec<u32> = LINE_FACTS
        .map(|a| ring[a[0]] as u32 * 100 + ring[a[1]] as u32 * 10 + ring[a[2]] as u32)
        .to_vec();
    let (min_index, _) = line_nums.iter().enumerate().fold(
        (usize::MIN, u32::MAX),
        |(min_index, min), (index, &val)| {
            if val < min {
                return (index, val);
            }
            (min_index, min)
        },
    );

    let mut eval: f64 = 0.0;
    for i in 0..line_nums.len() {
        let index = ((i + min_index) % line_nums.len()) as usize;
        let n = line_nums[index];
        let pow = 10_u64.pow(((n as f64).log10()) as u32) * 10;
        eval = eval * (pow as f64) + n as f64;
    }

    // penalty
    let mut line_sums = get_line_sums(ring);
    line_sums.sort();
    let penalty = PENALTY_WEIGHT * (line_sums.last().unwrap() - line_sums.first().unwrap()) as f64;

    //println!(
    //    "{:?} -> {} - {} = {}",
    //    line_sums,
    //    eval,
    //    penalty,
    //    eval - penalty
    //);

    eval - penalty
}

fn get_line_sums(ring: [u8; INDEX]) -> [u32; 5] {
    let mut line_sums = LINE_FACTS.map(|a| (ring[a[0]] + ring[a[1]] + ring[a[2]]) as u32);
    line_sums.sort();

    line_sums
}
