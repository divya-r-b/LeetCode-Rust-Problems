//sliding window approach
use std::collections::HashMap;
 impl Solution {
    pub fn length_of_longest_substring(s: String) -> i32 {
    
    let mut char_index_map: HashMap<char, usize> = HashMap::new();//store characters as keys and their corresponding indices in the string as values.
    let mut max_length = 0;//This variable keeps track of the maximum length of the substring without repeating characters encountered so far.
    let mut start_index = 0;//This variable keeps track of the start of the current substring without repeating characters.

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
