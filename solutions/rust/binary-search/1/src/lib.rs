pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mut start = 0usize;
    let mut end = array.len() - 1;

    while start <= end {
        let mid = start + ((end - start) / 2);
        if array[mid] == key {
            return Some(mid);
        }else if array[mid] > key && mid > 0{
            end = mid - 1;
        }else {
            start = mid + 1;
        }
    }

    None
}
