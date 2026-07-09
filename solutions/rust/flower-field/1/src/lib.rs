pub fn annotate(garden: &[&str]) -> Vec<String> {
    let space = ' ' as u8;
    let flower = '*' as u8;

    let mut sol = Vec::new();

    for (r, row) in garden.iter().enumerate() {
        let row_bytes = row.as_bytes();
        sol.push(row_bytes.to_owned());
        for (c, byte) in row_bytes.iter().enumerate() {
            if byte == &space {
                let mut cnt = 0;
                // check flowers at top, left, top-left, top-right
                if r>0 && sol[r-1][c] == flower {
                    // top
                    cnt += 1;
                }
                if r>0 && c>0 && sol[r-1][c-1] == flower {
                    // top-left
                    cnt += 1;
                }
                if r>0 && c+1 < sol[r-1].len() && sol[r-1][c+1] == flower {
                    // top-right
                    cnt += 1;
                }
                if c>0 && sol[r][c-1] == flower {
                    // left
                    cnt += 1;
                }
                
                // check if there is any flower at all at these four directions
                if cnt > 0 {
                    let digit = char::from_digit(cnt, 10).unwrap() as u8;
                    sol[r][c] = digit;
                }
            }else {
                // the byte is a flower
                // update spaces in top, left, top-left, top-right
                if r>0 && sol[r-1][c] != flower {
                    // top
                    if sol[r-1][c] == space {
                        sol[r-1][c] = '1' as u8;
                    } else {
                        // already a digit, so just increase it
                       sol[r-1][c] += 1;
                    }
                }
                if r>0 && c>0 && sol[r-1][c-1] != flower {
                    // top-left
                    if sol[r-1][c-1] == space {
                        sol[r-1][c-1] = '1' as u8;  
                    } else {
                        // already a digit, so just increase it
                        sol[r-1][c-1] += 1;
                    }
                }
                if r>0 && c+1 < sol[r-1].len() && sol[r-1][c+1] != flower {
                    // top-right
                    if sol[r-1][c+1] == space {
                        sol[r-1][c+1] = '1' as u8;  
                    } else {
                        // already a digit, so just increase it
                        sol[r-1][c+1] += 1;
                    }
                }
                if c>0 && sol[r][c-1] != flower {
                    // left
                    if sol[r][c-1] == space {
                        sol[r][c-1] = '1' as u8;  
                    } else {
                        // already a digit, so just increase it
                        sol[r][c-1] += 1;
                    }
                }
            }
        }
    }

    sol.into_iter().map(|r| String::from_utf8_lossy(&r).into_owned()).collect::<Vec<_>>()
}
