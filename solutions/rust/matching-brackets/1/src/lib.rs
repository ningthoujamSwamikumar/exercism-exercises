pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = Vec::new();
    // push for every encouter of opening bracket
    // pop when encounter a closing bracket and the stack top is opening bracket of same type
    // balance when finish iterating all the characters in the string, and stack is empty
    for c in string.chars() {
        match c {
            '[' => stack.push('['),
            '{' => stack.push('{'),
            '(' => stack.push('('),
            
            ']' => {
                if stack.last() != Some(&'[') {
                    return false;
                } 
                stack.pop();
            },
            '}' => {
                if stack.last() != Some(&'{') {
                    return false;
                } 
                stack.pop();
            },
            ')' => {
                if stack.last() != Some(&'(') {
                    return false;
                } 
                stack.pop();
            },

            _ => continue,
        }
    }

    stack.is_empty()
}
