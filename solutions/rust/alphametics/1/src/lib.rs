use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let words:Vec<String> = find_words(input);
    let non_zero_charset:HashSet<char> = non_zero_chars(&words);
    let mut taken_digits: HashSet<u8> = HashSet::new(); //0-9
    let mut map: HashMap<char, u8> = HashMap::new();
    let unique_chars: Vec<char> = unique_chars(&words).into_iter().collect();
    backtrack(0 as usize, &unique_chars, &mut taken_digits, &mut map, &non_zero_charset, &words)
}

fn backtrack(pos: usize, uniques: &Vec<char>, taken: &mut HashSet<u8>, map: &mut HashMap<char, u8>, non_zeroes: &HashSet<char>, words: &Vec<String>) -> Option<HashMap<char, u8>>{
    if pos == uniques.len() {
        if validate(words, &map) {
            return Some(map.clone());
        }
        return None;
    }
    for i in 0..10 {
        if i==0 && non_zeroes.contains(&uniques[pos]) {
            continue;
        }
        if taken.contains(&(i as u8)) {
            continue;
        }
        taken.insert(i);
        map.insert(uniques[pos], i);
        if let Some(ans) = backtrack(pos+1, uniques, taken, map, non_zeroes, words) {
            return Some(ans);
        }
        map.remove(&uniques[pos]);
        taken.remove(&i);
    }
    None
}

fn validate(words: &Vec<String>, map: &HashMap<char, u8>)->bool {
    let end = words.len() - 1;
    let mut sum = 0u64;
    for i in 0..end {
        let num = convert(&words[i], map).unwrap();
        sum += num;
    }
    let result = convert(&words[words.len()-1], map).unwrap();
    if sum == result {
        return true;
    }
    false
}

fn convert(word: &String, map: &HashMap<char, u8>)->Result<u64, ParseIntError> {
    let result = word.chars().map(|c| map.get(&c).unwrap().to_string()).collect::<Vec<String>>().join("");
    println!("{result}");
    result.parse::<u64>()
}

fn find_words(input: &str)->Vec<String>  {
    input.split(|c| c=='+' || c=='=')
    .map(|s| s.trim().to_string())
    .filter(|s| !s.is_empty())
    .collect::<Vec<String>>()
}

fn unique_chars(words: &Vec<String>)->HashSet<char>{
    let mut set = HashSet::new();
    words.iter().for_each(|w| w.chars().for_each(|c| { set.insert(c); }));
    set
}

fn non_zero_chars(words: &Vec<String>)->HashSet<char>{
    let mut set = HashSet::new();
    for word in words {
        if word.len() > 1 {
            set.insert(word.chars().next().unwrap());
        }
    }
    set
}
