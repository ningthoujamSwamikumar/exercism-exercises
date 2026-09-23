pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    // todo!("based on the {diagram}, determine the plants the {student} is responsible for");
    let children = ["Alice", "Bob", "Charlie", "David", "Eve", "Fred", "Ginny", "Harriet", "Ileana", "Joseph", "Kincaid", "Larry"]; // children in alphabetical order
    // find the turn (index) of the student given
    let Ok(student_turn) = children.binary_search(&student) else {
        return vec!(); 
    };

      //dbg!(diagram);
    // lets assume diagram is valid for all student provided
    // or else we need to validate it

    let second_row_offset = diagram.len() / 2;
    // now, [seed1, seed2, seed3, seed4] will be at [s_id*2, s_id*2+1, second_row_offset + s_id*2 + 1, second_row_offset + s_id * 2 + 2]
    
    let mut ans = vec![];

    for i in [student_turn*2, student_turn*2+1, second_row_offset + student_turn * 2 + 1, second_row_offset + student_turn * 2 + 2] {
        if let Some(seed) = diagram.get(i..i+1) {
            let seed_name = match seed {
                "V" => "violets",
                "R" => "radishes",
                "G" => "grass",
                "C" => "clover",
                _ => "unknown"
            };
            ans.push(seed_name);
        }
    }

    ans
}
