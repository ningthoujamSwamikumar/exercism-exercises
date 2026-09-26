pub fn egg_count(display_value: u32) -> usize {
    // todo!("count the eggs in {display_value}")
    let mut display_value = display_value;

    let mut count = 0_usize;
    while display_value > 0 {
        if display_value & 1 == 1 {
            count += 1;
        }
        display_value >>= 1;
    }

    count
}
