# 📌 Binary Search in Rust

## 🚀 Description
This Rust program implements the binary search algorithm to search for a specific element in a sorted array. Binary search efficiently finds the target element by repeatedly dividing the search range in half.

## 🔍 How It Works
### The function <mark>binary_search(arr: &[i32], target: i32) -> Option<usize></mark>:

- **Initial Setup**: The search range is defined by two pointers: <mark>left</mark> (starting at index 0) and <mark>right</mark> (starting at the last index of the array).

- #### Loop: The algorithm checks the middle element of the range:

    - If the middle element is the target, the index of that element is returned.

    - If the middle element is less than the target, the search range is adjusted to the right half.

    - If the middle element is greater than the target, the search range is adjusted to the left half.

- This process repeats until the target is found or the range is exhausted.

- **Return Value**: The function returns <mark>Some(index)</mark> if the element is found, and <mark>None</mark> if it's not found.

### Main Function:

- Defines a sorted <mark>array arr</mark> and calls the <mark>binary_search</mark> function to check if the target element (5) is present.

- The result is printed using a match expression to handle the <mark>Option</mark> returned by <mark>binary_search</mark>.

## 🎯 Example Output
```sh
Element found at index: 4
```

## 📂 Explanation
### <mark>binary_search Function</mark>:

- **Initialization**: The search range is defined by <mark>left</mark> and <mark>right</mark> pointers, which initially point to the first and last elements of the array, respectively.

- #### Loop: The search continues as long as <mark>left <= right</mark>. The middle element <mark>arr[mid]</mark> is checked:
       
   - If it's equal to the target, the index is returned.

   - If it's less than the target, the search continues in the right half of the array.

   - If it's greater than the target, the search continues in the left half.

- **Return**: If the target is found, the index is returned as <mark>Some(index)</mark>; otherwise, <mark>None</mark> is returned.

### Main Function:
- Defines a <mark>sorted array arr</mark> and searches for the element <mark>5</mark>.

- The <mark>binary_search</mark> function is called, and the result is printed based on whether the element was found or not.
