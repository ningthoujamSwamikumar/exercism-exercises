pub fn nth(n: u32) -> u32 {
    // given n is 0 based i.e. 1 would be given as 0, 2 would be 1, ...
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

    let mut count = 4; // 0 based indexing/numbering
    for i in (13..).step_by(2) {
        print!("checking {i} ...");
        if is_prime(i) {
            println!("its prime.");
            count += 1;
        } else {
            println!("its not prime.");
        }
        if count == n {
            return i;
        }
    }

    0
}

fn is_prime(num: u32) -> bool {
    if num <= 1 {
        return false;
    };
    if num <= 3 {
        return true;
    };
    if num % 2 == 0 || num % 3 == 0 {
        // multiples of 2 and 3, or 6
        return false;
    }

    // check factors 6k +/- 1 upto sqrt
    // the loop is not running for num 25, as its sqrt is 5, 
    // hence 25 is consider prime
    // fix: lets check upto one more number than the sqrt
    for i in (6..=(num.isqrt() + 1)).step_by(6) {
        if num % (i - 1) == 0 || num % (i + 1) == 0 {
            // num is multiple of 6k-1 or 6k+1
            return false;
        }
    }

    true
}
