pub fn is_armstrong_number(num: u32) -> bool {
    let mut digits: Vec<u32> = vec![];
    
    let mut mut_num = num;
    while mut_num > 0 {
        let digit = mut_num % 10;
        mut_num = mut_num / 10;
        digits.push(digit);
    }

    let mut sum = 0;
    for d in digits.iter() {
        sum += d.pow(digits.len() as u32);
    }

    sum == num
}
