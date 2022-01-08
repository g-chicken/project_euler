fn main() {
    let v = &mut Vec::<u8>::new();
    permutation(v, 10, 0, 0);
}

fn permutation(used_num: &mut Vec<u8>, max: u8, p: u32, order: u32) -> u32 {
    let mut o: u32 = order;

    for i in 0..max {
        if !used_num.contains(&i) {
            used_num.push(i);
            o = permutation(used_num, max, p * 10 + i as u32, o);

            if o == 1000000 {
                return o;
            }

            used_num.pop();
        }
    }

    if used_num.len() == max.into() {
        if o + 1 == 1000000 {
            println!("{}: {}", o + 1, p);
        }

        return o + 1;
    }

    o
}
