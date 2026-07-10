pub fn build_proverb(list: &[&str]) -> String {
    if list.is_empty() {
        return String::new();
    }
    
    let mut proverbs: Vec<String> = Vec::new();
    if list.len() > 1 {
        for i in 0..=list.len()-2 {
            proverbs.push(format!("For want of a {} the {} was lost.", list[i], list[i+1]));
        }
    }
    proverbs.push(format!("And all for the want of a {}.", list[0]));

    proverbs.join("\n")
}
