# 📌 Remove Duplicates from a Sorted Array in Rust
## 🚀 Description
This Rust program removes duplicates from a sorted vector in-place and returns the new length of the modified vector. 
The function ensures that **each unique element appears only once** while maintaining the original order.

## 🔍 How It Works
### Key Steps:
#### 1. Handle Empty Vector:

   - If the input vector is empty, return <mark>0</mark>.

Use Two Pointers:

unique_idx keeps track of the position for unique elements.

The loop iterates through the array, comparing each element to the last unique element.

Modify the Vector in Place:

When a new unique element is found, move it to the correct position.

Return the New Length:

The function returns unique_idx + 1, which represents the number of unique elements.

🎯 Example Output
sh
Copy
Edit
Array after duplicates removed: [1, 2, 3, 4]
📂 Explanation
remove_duplicates Function:
Uses two pointers:

unique_idx to track unique elements.

i to iterate through the vector.

If the current element is different from the last unique element, it's placed at unique_idx + 1.

The function returns the new length (unique_idx + 1).

main Function:
Initializes a vector with duplicates.

Calls remove_duplicates() to modify the vector in place.

Prints the unique elements using slicing (&nums[..new_len]).
