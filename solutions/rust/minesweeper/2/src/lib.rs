pub fn annotate(minefield: &[&str]) -> Vec<String> {
    // todo!(
    //     "\nAnnotate each square of the given minefield with the number of mines that surround said square (blank if there are no surrounding mines):\n{minefield:#?}\n"
    // );

    let mut board_count:Vec<String> = Vec::new();
    for (i, curr) in minefield.iter().enumerate() { 
        let mut row_count = String::new();
        let curr_bytes = curr.as_bytes();
        for (j, c) in curr_bytes.iter().enumerate() {
            if *c != b' ' {
                row_count.push(*c as char);
                continue;
            }
            let mut count = 0;
            //right
            if j+1 < curr_bytes.len() && curr_bytes[j+1] == b'*' {
                count += 1;
            }
            //left
            if j > 0 && curr_bytes[j-1] == b'*' {
                count += 1;
            }
            //top
            if i > 0 && minefield[i-1].as_bytes()[j] == b'*' {
                count += 1;
            }
            //down
            if i+1 < minefield.len() && minefield[i+1].as_bytes()[j] == b'*' {
                count += 1;
            }
            //top-right diagonal
            if i > 0 && j+1 < curr.len() && minefield[i-1].as_bytes()[j+1] == b'*' {
                count += 1;
            }
            //top-left diagonal
            if i > 0 && j > 0 && minefield[i-1].as_bytes()[j-1] == b'*' {
                count += 1;
            }
            //bottom-right diagonal
            if i+1 < minefield.len() && j+1 < curr.len() && minefield[i+1].as_bytes()[j+1] == b'*' {
                count += 1;
            }
            //bottom-left diagonal
            if i+1 < minefield.len() && j>0 && minefield[i+1].as_bytes()[j-1] == b'*' {
                count += 1;
            }

            if count == 0 {
                row_count.push(' ');
            }else{
                row_count.push_str(&count.to_string());
            }
        }
        board_count.push(row_count);
    }
    //println!("{:?}", board_count);
    board_count
}
