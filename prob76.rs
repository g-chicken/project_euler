const LIMIT: usize = 101;

fn main() {
    let mut dp: [[u32; LIMIT]; LIMIT] = [[0; LIMIT]; LIMIT];
    let mut n = 2;

    while n < dp.len() {
        let mut m = 1;
        while n - m >= m {
            let mut count = 1;
            for i in m..dp[n - m].len() {
                count += dp[n - m][i];
            }
            dp[n][m] = count;
            m += 1;
        }

        n += 1;
    }

    let mut count = 0;
    for i in 1..dp.len() {
        print!("{:>3} : ", i);
        for j in 1..dp[i].len() {
            print!("{:>5} ", dp[i][j]);

            if i == dp.len() - 1 {
                count += dp[i][j];
            }
        }
        println!();
    }

    println!("{}", count);
}
