fn merge_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    
    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);
    
    let mut result = arr.to_vec();
    merge(&arr[0..mid], &arr[mid..], &mut result);
    arr.copy_from_slice(&result);
}

fn merge<T: Ord>(left: &[T], right: &[T], result: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result[k] = left[i].clone();
            i += 1;
        } else {
            result[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        result[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        result[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

fn main() {
    let mut arr = [5, 2, 9, 1, 5, 6];
    merge_sort(&mut arr);
    println!("Sorted Array: {:?}", arr);  // Output: [1, 2, 5, 5, 6, 9]
}
