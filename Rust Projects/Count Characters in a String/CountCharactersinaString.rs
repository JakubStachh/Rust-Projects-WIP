use std::collections::HashMap;

fn count_characters(s: &str) -> HashMap<char, i32> {
    let mut char_count = HashMap::new();
    
    for ch in s.chars() {
        *char_count.entry(ch).or_insert(0) += 1;
    }
    
    char_count
}

fn main() {
    let s = "hello world";
    let counts = count_characters(s);
    
    for (ch, count) in counts {
        println!("Character '{}' appears {} times", ch, count);
    }
}
