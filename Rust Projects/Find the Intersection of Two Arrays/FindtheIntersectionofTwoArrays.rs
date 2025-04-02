use std::collections::HashSet;

fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let set1: HashSet<_> = nums1.into_iter().collect();
    let set2: HashSet<_> = nums2.into_iter().collect();
    
    set1.intersection(&set2).cloned().collect()
}

fn main() {
    let nums1 = vec![1, 2, 2, 1];
    let nums2 = vec![2, 2];
    println!("Intersection: {:?}", intersection(nums1, nums2)); // Output: [2]
}
