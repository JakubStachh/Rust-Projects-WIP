use std::cmp::Ord;

fn merge_sort<T: Ord + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    
    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    // Create a result vector to store merged values
    let mut result = arr.to_vec();
    merge(&arr[0..mid], &arr[mid..], &mut result);
    arr.copy_from_slice(&result); // This works now because `T` implements `Copy`
}

fn merge<T: Ord + Copy>(left: &[T], right: &[T], result: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i]; // Directly copy the value because `T` implements `Copy`
            i += 1;
        } else {
            result[k] = right[j];
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i];
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j];
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    merge_sort(&mut arr);
    println!("Sorted Array: {:?}", arr);  // Output: [1, 2, 5, 5, 6, 9]
}
