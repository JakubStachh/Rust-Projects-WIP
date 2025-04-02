# 📌 Merge Sort in Rust

## 🚀 Description
This Rust program implements the merge sort algorithm. 
Merge sort is a divide-and-conquer algorithm that breaks an array into smaller subarrays, sorts them, and then merges the sorted subarrays. 
This implementation is generic and can be used with any type that implements the `Ord` and `Copy` traits.

## 🔍 How It Works
### Divide:

- The array is recursively divided into two halves until each subarray contains one or zero elements (base case).

### Merge:

- Two sorted halves are merged into a sorted array by comparing the elements one by one.

### Copy:

- The elements are copied back into the original array after merging.

## Key Points:
- **Generics**: The `merge_sort` and `merge` functions are generic over types `T` that implement the `Ord` and `Copy` traits, which means you can sort any array of types that can be compared and copied.

- `Ord`: This trait is used to define a total order on the elements of the array, which is necessary for comparison during sorting.

- `Copy`: This trait ensures that elements can be copied (not moved) when merging arrays, enabling efficient sorting with minimal overhead.

## 🎯 Example Output
```sh
Sorted Array: [1, 2, 5, 5, 6, 9]
```

## 📂 Explanation
### `merge_sort` Function:
- **Recursion**:

   - The array is divided into two halves recursively using the midpoint (`mid`).

- **Base Case**:

   - If the array has fewer than two elements, it’s already sorted, so the function returns immediately.

- **Merging**:

    - After sorting the two halves, the `merge` function is called to merge them back together in sorted order.

- **Copying**:

    - The merged result is copied back to the original array using `arr.copy_from_slice(&result)`.

### `merge` Function:
- **Merging Logic**:

    - The two sorted subarrays (`left` and `right`) are merged by comparing the elements one by one.

    - The smaller element is added to the result array first.

    - If there are remaining elements in either subarray, they are copied to the result.

### `main` Function:
- A sample array `[5, 2, 9, 1, 5, 6]` is provided.

- The `merge_sort` function is called to sort the array.

- The sorted array is printed.
