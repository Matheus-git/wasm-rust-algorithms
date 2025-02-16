pub fn binary_search(x: i32, max: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = max; 
    let mut steps = 0;

    while left <= right {
        steps += 1;
        let middle_index = (left + right) / 2;
        let middle_value = middle_index;

        if x == middle_value {
            return Some(steps);
        } else if x < middle_value {
            right = middle_index -1;
        } else {
            left = middle_index + 1;
        }
    }

    None
}
