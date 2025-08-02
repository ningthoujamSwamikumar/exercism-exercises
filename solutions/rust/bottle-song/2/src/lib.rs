
const NUMBER_WORDS:[&str; 11] = ["no", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten"];

pub fn recite(start_bottles: u32, take_down: u32) -> String {
    // todo!("Return the bottle song starting at {start_bottles} and taking down {take_down} bottles")

    let mut ans = String::new();
    let end_number = start_bottles - take_down + 1;

    ans.push_str(&chorus(start_bottles));
    
    for b in ((end_number)..(start_bottles)).rev() {
        println!("inserting {b}th time");
        ans.push_str("\n\n");
        ans.push_str(&chorus(b));
    }
    println!("ans: \\n{ans}");

    ans
}

fn chorus(start_bottles: u32)->String {
    let capitalize_start = capitalize_first(NUMBER_WORDS[start_bottles as usize]);
    println!("start_bottles in chorus: {start_bottles}");
    let next = NUMBER_WORDS[(start_bottles - 1u32) as usize];
    if start_bottles ==  1 {
        format!("{capitalize_start} green bottle hanging on the wall,\n{capitalize_start} green bottle hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {next} green bottles hanging on the wall.")
    }else if start_bottles == 2 {
        format!("{capitalize_start} green bottles hanging on the wall,\n{capitalize_start} green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {next} green bottle hanging on the wall.")
    }else{
        format!("{capitalize_start} green bottles hanging on the wall,\n{capitalize_start} green bottles hanging on the wall,\nAnd if one green bottle should accidentally fall,\nThere'll be {next} green bottles hanging on the wall.")
    }
}

fn capitalize_first(word: &str)->String {
    let mut chars = word.chars();
    match chars.next() {
        None => String::new(),
        Some(first)=>first.to_uppercase().collect::<String>() + chars.as_str(),
    }
}
