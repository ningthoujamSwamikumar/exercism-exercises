pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
    2_u64.pow(s-1) 
}

pub fn total() -> u64 {
    // todo!();
    //2_u64.pow(64) - 1 //overflow error
    //(1_u128 << 1) - 1 //type error but solves overflow error
    u64::MAX
}
