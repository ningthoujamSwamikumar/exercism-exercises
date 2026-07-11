pub fn factors(n: u64) -> Vec<u64> {
    let mut n = n;
    let mut p_factors: Vec<u64> = Vec::new();
    let mut d: u64 = 2;
    while n > 1 {
        if n.is_multiple_of(d) {
            n /= d;
            p_factors.push(d);
        }else{
            d += 1;
        }
    }

    p_factors
}
