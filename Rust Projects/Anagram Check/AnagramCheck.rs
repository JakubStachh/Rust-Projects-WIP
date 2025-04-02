use std::collections::HashMap;

fn are_anagrams(s1: &str, s2: &str) -> bool {
    let mut map1 = HashMap::new();
    let mut map2 = HashMap::new();
    
    for ch in s1.chars() {
        *map1.entry(ch).or_insert(0) += 1;
    }
    
    for ch in s2.chars() {
        *map2.entry(ch).or_insert(0) += 1;
    }
    
    map1 == map2
}

fn main() {
    println!("{}", are_anagrams("listen", "silent"));  // Output: true
    println!("{}", are_anagrams("hello", "world"));    // Output: false
}
