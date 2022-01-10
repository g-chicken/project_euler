fn main() {
    let mut pentagonal_numbers: [u32; 5000] = [0; 5000];

    for n in 1..pentagonal_numbers.len() as u32 {
        pentagonal_numbers[n as usize] = n * (3 * n - 1) / 2;
    }

    let mut terminate: bool = false;
    for i in 2..pentagonal_numbers.len() {
        for j in 1..i {
            let p_sum: u32 = pentagonal_numbers[i] + pentagonal_numbers[j];
            let p_sub: u32 = pentagonal_numbers[i] - pentagonal_numbers[j];

            if pentagonal_numbers.to_vec().contains(&p_sum)
                && pentagonal_numbers.to_vec().contains(&p_sub)
            {
                println!(
                    "P_{} - P_{} = {} - {} = {}",
                    i,
                    j,
                    pentagonal_numbers[i],
                    pentagonal_numbers[j],
                    pentagonal_numbers[i] - pentagonal_numbers[j],
                );

                terminate = true;
                break;
            }
        }

        if terminate {
            break;
        }
    }
}
