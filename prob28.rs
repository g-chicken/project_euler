fn main() {
    let square: u32 = 1001;
    let n: u32 = (square + 1) / 2;
    let mut left_up_sum: u32 = 0;
    let mut left_down_sum: u32 = 0;
    let mut right_up_sum: u32 = 0;
    let mut right_down_sum: u32 = 0;

    for i in 1..n as usize {
        let i_u32: u32 = i as u32;

        left_up_sum += (2 * i_u32 + 1) * (2 * i_u32 + 1);
        right_up_sum += (2 * i_u32 + 1) * (2 * i_u32 + 1) - 2 * i_u32;
        right_down_sum += (2 * i_u32 + 1) * (2 * i_u32 + 1) - 2 * i_u32 * 2;
        left_down_sum += (2 * i_u32 + 1) * (2 * i_u32 + 1) - 2 * i_u32 * 3;
    }

    println!(
        "{}",
        left_up_sum + left_down_sum + right_up_sum + right_down_sum + 1
    );
}
