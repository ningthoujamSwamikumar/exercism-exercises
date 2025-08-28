pub fn abbreviate(phrase: &str) -> String {
    //todo!("Given the phrase '{phrase}', return its acronym");

    if phrase.is_empty() {
        return String::new();
    }
    let formatted_phrase = phrase.trim().replace("-", " ")
                    .replace(&['.', '_', ':', ';'][..], "");
    
    let mut acronym = String::new();
    
    for word in formatted_phrase.split_whitespace() {
        let mut char_iter = word.chars();
        let c1 = char_iter.next().unwrap()
                    .to_uppercase().next().unwrap();
        acronym.push(c1);
        
        if word.chars().all(|c| c.is_uppercase()) {
            continue;
        }
        
        char_iter.for_each(|c| if c.is_uppercase() {
            acronym.push(c);
        });
    }

    acronym
}
