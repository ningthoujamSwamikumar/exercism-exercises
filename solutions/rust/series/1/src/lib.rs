pub fn series(digits: &str, len: usize) -> Vec<String> {
    //todo!("What are the series of length {len} in string {digits:?}")
    let mut substrings = Vec::new();

    if digits.len() < len {
        return substrings;
    }

    let mut window_start = 0;
    while let Some(substr) = digits.get(window_start..(window_start+len)) {
        substrings.push(substr.into());
        window_start += 1;
    }

    substrings
}
