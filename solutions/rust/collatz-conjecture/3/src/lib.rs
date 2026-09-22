pub fn collatz(n: u64) -> Option<u64> {
    //todo!("return Some(x) where x is the number of steps required to reach 1 starting with {n}")
    if n == 0 {
        return None;
    }

    let mut count = 0u64;
    let mut n = n;
    while n > 1 {
        if n.is_multiple_of(2) {
            n /= 2;
        } else {
            n *= 3;
            n += 1;
        }
        count += 1;
    }    

    if n == 1 {
        return Some(count);
    }

    None
}
