fn main() {
    let letters: [u32; 21] = [0, 3, 3, 5, 4, 4, 3, 5, 5, 4, 3, 6, 6, 8, 8, 7, 7, 9, 8, 8, 6];
    let letter10: [u32; 10] = [0, 3, 6, 6, 5, 5, 5, 7, 6, 6];
    let letter100: u32 = 7;
    let mut sum: u32 = 11; // one thousand

    for n in 1..1000 {
        let mut s: u32 = 0;
        let digit3 = n / 100;
        let digit2 = (n / 10) % 10;
        let digit1 = n % 10;

        if digit3 > 0 {
            s += letters[digit3] + letter100;

            if digit2 != 0 || digit1 != 0 {
                s += 3;  // and
            }
        }

        if n % 100 <= 20 {
            s += letters[n % 100];
        } else {
            s += letter10[digit2] + letters[digit1];
        }

        sum += s;
    }

    println!("{}", sum);
}
