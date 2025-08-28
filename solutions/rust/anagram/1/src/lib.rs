use std::collections::{ HashSet, HashMap};

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&str]) -> HashSet<&'a str> {
    let mut set:HashSet<&str> = HashSet::new();

    let mut word_map: HashMap<char, u8> = HashMap::new();
    for ch in word.to_lowercase().chars() {
        word_map.insert(ch, word_map.get(&ch).unwrap_or(&0) + 1 );
    }
    for candidate in possible_anagrams {
        if candidate.to_lowercase() == word.to_lowercase() {
            continue;
        }
        let mut candidate_map = HashMap::new();
        //construct map
        for ch in candidate.to_lowercase().chars() {
            candidate_map.insert(ch, candidate_map.get(&ch).unwrap_or(&0) + 1);
        }
        //compare two maps
        for (key, value) in word_map.iter() {
            match candidate_map.get(key) {
                Some(val)=> {
                    if val != value {
                        break;
                    }else{
                        candidate_map.remove(key);
                    }
                },
                None=> {
                    break;
                }
            }
        }
        //if all the entries are equal
        if candidate_map.len() == 0 {
            set.insert(candidate);
        }
    }

    set
}
