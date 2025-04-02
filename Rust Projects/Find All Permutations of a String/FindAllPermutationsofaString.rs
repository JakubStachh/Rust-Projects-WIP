fn permute(s: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut chars = s.chars().collect::<Vec<char>>();
    permute_helper(&mut chars, 0, &mut result);
    result
}

fn permute_helper(s: &mut Vec<char>, start: usize, result: &mut Vec<String>) {
    if start == s.len() {
        result.push(s.iter().collect());
        return;
    }
    
    for i in start..s.len() {
        s.swap(start, i);
        permute_helper(s, start + 1, result);
        s.swap(start, i); // Backtrack
    }
}

fn main() {
    let s = "abc";
    let permutations = permute(s);
    println!("{:?}", permutations); // Output: ["abc", "acb", "bac", "bca", "cab", "cba"]
}
