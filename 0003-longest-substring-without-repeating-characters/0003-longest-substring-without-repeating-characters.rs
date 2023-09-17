 use std::collections::HashMap;
 impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    
    let mut char_index_map: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut start_index = 0;

    for (end_index, c) in s.chars().enumerate() {
        if let Some(&prev_index) = char_index_map.get(&c) {
            start_index = start_index.max(prev_index + 1);
        }

        char_index_map.insert(c, end_index);
        max_length = max_length.max(end_index - start_index + 1);
    }

    max_length as i32
}   
    }