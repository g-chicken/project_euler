fn main() {
    // 1 Jan 1901 = 1
    // 7 (7 Jan 1901) is Sunday.
    // 1 Jan. -> 1 mod 365
    // 1 Feb. -> 32 mod 365
    // 1 Mar. -> 60 mod 365
    // 1 Apr. -> 91 mod 365
    // 1 May. -> 121 mod 365
    // 1 Jun. -> 152 mod 365
    // 1 Jul. -> 182 mod 365
    // 1 Aug. -> 213 mod 365
    // 1 Sep. -> 244 mod 365
    // 1 Oct. -> 274 mod 365
    // 1 Nob. -> 305 mod 365
    // 1 Dec. -> 335 mod 365

    const DATE_IN_YEAR: u64 = 365;

    let mut sunday: u64 = 7;
    let mut last_leap_year: u64 = 0;
    let mut leap_year_num: u64 = 0;
    let mut result: u32 = 0;

    loop {
        let mut year: u64 = (sunday - leap_year_num) / DATE_IN_YEAR + 1900;
        let mut date: u64 = (sunday - leap_year_num) % DATE_IN_YEAR;

        if date > 59 && year != last_leap_year {
            if year % 100 == 0 && year % 400 != 0 {
                // nothing to do
            } else if year % 4 == 0 {
                leap_year_num += 1;
                year = (sunday - leap_year_num) / DATE_IN_YEAR + 1900;
                date = (sunday - leap_year_num) % DATE_IN_YEAR;
                last_leap_year = year;
            }
        }

        if year > 2000 {
            break;
        }

        if year > 1900 {
            match date {
                1 | 32 | 60 | 91 | 121 | 152 | 182 | 213 | 244 | 274 | 305 | 335 => {
                    result += 1;
                }
                _ => {
                    // nothing to do
                }
            }
        }

        sunday += 7;
    }

    println!("{}", result);
}
