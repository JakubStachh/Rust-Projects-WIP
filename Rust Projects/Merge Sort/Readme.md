# 📌 Merge Sort in Rust

## 🚀 Description
This Rust program implements the merge sort algorithm. 
Merge sort is a divide-and-conquer algorithm that breaks an array into smaller subarrays, sorts them, and then merges the sorted subarrays. 
This implementation is generic and can be used with any type that implements the <mark>Ord</mark> and <mark>Copy</mark> traits.

## 🔍 How It Works
### Divide:

- The array is recursively divided into two halves until each subarray contains one or zero elements (base case).

### Merge:

- Two sorted halves are merged into a sorted array by comparing the elements one by one.

### Copy:

- The elements are copied back into the original array after merging.

## Key Points:
- **Generics**: The <mark>merge_sort</mark> and <mark>merge</mark> functions are generic over types <mark>T</mark> that implement the <mark>Ord</mark> and <mark>Copy</mark> traits, which means you can sort any array of types that can be compared and copied.

- <mark>Ord</mark>: This trait is used to define a total order on the elements of the array, which is necessary for comparison during sorting.

- <mark>Copy</mark>: This trait ensures that elements can be copied (not moved) when merging arrays, enabling efficient sorting with minimal overhead.

## 🎯 Example Output
```sh
Sorted Array: [1, 2, 5, 5, 6, 9]
```

## 📂 Explanation
### <mark>merge_sort</mark> Function:
- **Recursion**:

   - The array is divided into two halves recursively using the midpoint (<mark>mid</mark>).

- **Base Case**:

   - If the array has fewer than two elements, it’s already sorted, so the function returns immediately.

- **Merging**:

    - After sorting the two halves, the <mark>merge</mark> function is called to merge them back together in sorted order.

- **Copying**:

    - The merged result is copied back to the original array using <mark>arr.copy_from_slice(&result)</mark>.

### <mark>merge</mark> Function:
- **Merging Logic**:

    - The two sorted subarrays (<mark>left</mark> and <mark>right</mark>) are merged by comparing the elements one by one.

    - The smaller element is added to the result array first.

    - If there are remaining elements in either subarray, they are copied to the result.

### <mark>main</mark> Function:
- A sample array <mark>[5, 2, 9, 1, 5, 6]</mark> is provided.

- The <mark>merge_sort</mark> function is called to sort the array.

- The sorted array is printed.
