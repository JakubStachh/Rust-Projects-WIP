fn find_missing_number(arr: Vec<i32>, n: i32) -> i32 {
    let expected_sum = n * (n + 1) / 2;
    let actual_sum: i32 = arr.iter().sum();
    expected_sum - actual_sum
}

fn main() {
    let arr = vec![1, 2, 4, 5, 6];
    let n = 6;
    println!("The missing number is: {}", find_missing_number(arr, n)); // Output: 3
}
