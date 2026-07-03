pub fn is_leap_year(year: u64) -> bool {
    // every year divisible by 4 is a leap year, unless its divisible by 100
    // if its divisible by 100, its a leap year only if its divisible by 400 e.g. 200
    if year % 100_u64 == 0 {
        if year % 400_u64 == 0 {
            return true;
        } else {
            return false;
        };
    };
    year % 4_u64 == 0
}
