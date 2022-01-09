fn main() {
    let coin: [u16; 8] = [200, 100, 50, 20, 10, 5, 2, 1];

    println!("{}", use_coin(&coin, 0, 0, 0));
}

fn use_coin(coin: &[u16; 8], sum: u16, index: usize, ways: u32) -> u32 {
    if index == coin.len() {
        if sum == 200 {
            return ways + 1;
        }

        return ways;
    }

    let mut w: u32 = ways;

    let mut i: u16 = 0;
    while sum + coin[index] * i <= 200 {
        w = use_coin(coin, sum + coin[index] * i, index + 1, w);

        i += 1;
    }

    w
}
