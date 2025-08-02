#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

use Comparison::*;

pub fn sublist(first_list: &[i32], second_list: &[i32]) -> Comparison {
    
    let (l1, l2) = (first_list.len(), second_list.len());
    if l1 == 0 && l1 == l2 {
        return Equal;
    }
    let (short, long) = if l1 <= l2 {
        (first_list, second_list)
    }else{
        (second_list, first_list)
    };

    //sliding window technique
    //scan the longer slice
    for i in 0..long.len() {
        //scan the shorter slice
        let mut k = i;
        for j in 0..short.len() {
            if long[k] != short[j] {
                break;
            }
            k+=1;
            if k == long.len() {
                //long ran out of bound
                break;
            }
        }
        if k-i != short.len() {
            continue;
        }
        //found equality
        if l1 == l2 {
            return Equal;
        }
        if l1 < l2 {
            return Sublist;
        }
        return Superlist;
    }

    Unequal
}
