impl Solution {
    pub fn longest_palindrome(s: String) -> String {
        if s.is_empty() {
            return String::new();
        }

        // Preprocess the input string to handle even-length palindromes.
        let mut modified_s = String::new();
        modified_s.push('^');
        for c in s.chars() {
            modified_s.push('#');
            modified_s.push(c);
        }
        modified_s.push('#');
        modified_s.push('$');

        let chars: Vec<char> = modified_s.chars().collect();
        let n = chars.len();
        let mut p = vec![0; n];
        let mut center = 0;
        let mut right = 0;
        let mut max_len = 0;
        let mut max_len_center = 0;

        for i in 1..n - 1 {
            if i < right {
                let mirror = 2 * center - i;
                p[i] = (right - i).min(p[mirror]);
            }

            // Attempt to expand palindrome centered at i.
            while chars[i + p[i] + 1] == chars[i - p[i] - 1] {
                p[i] += 1;
            }

            // If this palindrome's right edge exceeds the rightmost known palindrome,
            // update center and right.
            if i + p[i] > right {
                center = i;
                right = i + p[i];
            }

            // Update the maximum palindrome length and its center.
            if p[i] > max_len {
                max_len = p[i];
                max_len_center = i;
            }
        }

        // Extract the longest palindromic substring from the modified string.
        let start = (max_len_center - max_len) / 2;
        let end = start + max_len;
        s[start..end].to_string()
    }
}
