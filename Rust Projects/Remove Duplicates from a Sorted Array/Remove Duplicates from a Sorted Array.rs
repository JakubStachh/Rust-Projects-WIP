fn remove_duplicates(nums: &mut Vec<i32>) -> usize {
    if nums.is_empty() {
        return 0;
    }
    
    let mut unique_idx = 0;
    
    for i in 1..nums.len() {
        if nums[i] != nums[unique_idx] {
            unique_idx += 1;
            nums[unique_idx] = nums[i];
        }
    }
    
    unique_idx + 1
}

fn main() {
    let mut nums = vec![1, 1, 2, 2, 3, 4, 4];
    let new_len = remove_duplicates(&mut nums);
    println!("Array after duplicates removed: {:?}", &nums[..new_len]);
}
