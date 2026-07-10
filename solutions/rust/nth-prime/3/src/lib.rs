pub fn nth(n: u32) -> u32 {
    if n == 0 {
        return 2;
    }
    if n == 1 {
        return 3;
    }
    if n == 2 {
        return 5;
    }
    if n == 3 {
        return 7;
    }
    if n == 4 {
        return 11;
    }
    
    let mut count = 4;
    for i in (13..).step_by(2) {
        if is_prime(i) {
            count += 1;
        }
        if count == n {
            return i;
        }
    }

    0
}

fn is_prime(num: u32)->bool {
    if num <= 1 { return false };
    if num <= 3 { return true };
    if num.is_multiple_of(2) || num.is_multiple_of(3) {
        // multiples of 2 and 3, or 6
        return false;
    }

    // check factors 6k +/- 1 upto sqrt
    for i in (6..=(num.isqrt()+1)).step_by(6) {
        if num.is_multiple_of(i-1) || num.is_multiple_of(i+1) {
            // num is multiple of 6k-1 or 6k+1
            return false;
        }
    }

    true
}
