use std::collections::HashMap;

fn length_of_longest_substring(s: &str) -> usize {
    let mut map = HashMap::new();
    let mut max_len = 0;
    let mut left = 0;
    
    for (right, c) in s.chars().enumerate() {
        if let Some(&prev_idx) = map.get(&c) {
            left = left.max(prev_idx + 1);
        }
        map.insert(c, right);
        max_len = max_len.max(right - left + 1);
    }
    
    max_len
}

fn main() {
    let s = "abcabcbb";
    println!("Longest substring length: {}", length_of_longest_substring(s)); // Output: 3
}
