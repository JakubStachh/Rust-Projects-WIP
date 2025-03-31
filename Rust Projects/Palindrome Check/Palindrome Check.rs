fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<char>>();
    let len = s.len();
    
    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }
    true
}

fn main() {
    println!("{}", is_palindrome("racecar"));  // Output: true
    println!("{}", is_palindrome("hello"));    // Output: false
}
