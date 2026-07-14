pub fn reply(message: &str) -> &str {
    let message = message.trim();
    // When at silence
    if message.chars().all(|c| c.is_whitespace()) {
        return "Fine. Be that way!";
    }

    // check if the message has some letters in it, and all the letters are uppercase
    let is_uppercase = message.chars().any(|c| c.is_alphabetic()) && message.chars().all(|c| {
        if c.is_alphabetic() {
            c.is_uppercase()
        }else{
            true
        }
    });
    
    // YELL AT HIM
    if is_uppercase {
        // and ask question
        if message.ends_with("?") {
            return "Calm down, I know what I'm doing!";
        }else{
        // only yell
            return "Whoa, chill out!";
        }
    }
    // question him
    if message.ends_with("?") {
            return "Sure.";
    }

    "Whatever."
}
