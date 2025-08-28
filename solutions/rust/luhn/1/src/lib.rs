/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //todo!("Is the Luhn checksum for {code} valid?");
    //lets remove spaces
    let mut space_trimmed_code = String::new();
    code.split_whitespace()
        .into_iter()
        .for_each(|s|space_trimmed_code.push_str(s));

    //println!("space_trimmed_code: {space_trimmed_code}");

    if space_trimmed_code.len() <= 1 {
        return false;
    }
    
    let mut sum = 0;
    let mut is_second:bool = false;
    for d in space_trimmed_code.chars().rev() {
        //digit to add to digit sum
        if !char::is_numeric(d) {
            return false;
        }
        let mut num = d.to_digit(10).unwrap();
        if is_second {
            num *= 2;
            if num > 9 {
                num -= 9;
            }
        }
        sum += num;
        is_second = !is_second;
    }

    print!("sum: {sum}");
    
    sum % 10 == 0
}
