use std::collections::HashMap;  // Import the HashMap type from the standard library

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // Create a new HashMap to store the numbers and their indices.
        let mut map = HashMap::with_capacity(nums.len());
        
        // Iterate through the elements of the input vector `nums` along with their indices.
        for (index, num) in nums.iter().enumerate() {
            // Calculate the complement of the current number with respect to the target.
            let complement = target - num;
            
            // Check if the complement exists in the HashMap.
            match map.get(&complement) {
                None => {
                    // If the complement doesn't exist in the HashMap, insert the current number
                    // into the HashMap along with its index.
                    map.insert(num, index);
                }
                Some(sub_index) => {
                    // If the complement does exist in the HashMap, return a vector containing the
                    // indices of the two numbers that add up to the target.
                    return vec![*sub_index as i32, index as i32];
                }
            }
        }
        
        // If no solution is found, return an empty vector.
        vec![]
    }
}

//This code is an efficient solution to the "Two Sum" problem, as it uses a HashMap to store the numbers and their indices, allowing for constant-time lookups. It has a time complexity of O(n) since it iterates through the input vector once.