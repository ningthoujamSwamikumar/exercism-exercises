use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut set: HashSet<u32> = HashSet::new();
    factors.iter().for_each(|&f| {
        if f == 0 {
            // zero doesn't affect the sum of the unique multiples
            return ();
        }

        (f..limit).step_by(f as usize).for_each(|m| {
            set.insert(m);
        });
    });

    set.iter().sum()
}
