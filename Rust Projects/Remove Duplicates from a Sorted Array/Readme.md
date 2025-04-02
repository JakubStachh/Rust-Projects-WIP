# 📌 Remove Duplicates from a Sorted Array in Rust
## 🚀 Description
This Rust program removes duplicates from a sorted vector in-place and returns the new length of the modified vector. 
The function ensures that **each unique element appears only once** while maintaining the original order.

## 🔍 How It Works
### Key Steps:
#### 1. Handle Empty Vector:

   - If the input vector is empty, return <mark>0</mark>.

#### 2. Use Two Pointers:

   - <mark>unique_idx</mark> keeps track of the position for unique elements.

   - The loop iterates through the array, comparing each element to the last unique element.

#### 3. Modify the Vector in Place:

   - When a new unique element is found, move it to the correct position.

#### 4. Return the New Length:

   - The function returns <mark>unique_idx + 1</mark>, which represents the number of unique elements.

## 🎯 Example Output
```sh
Array after duplicates removed: [1, 2, 3, 4]
```

## 📂 Explanation
### <mark>remove_duplicates</mark> Function:
 - Uses **two pointers**:

   - <mark>unique_idx</mark> to track unique elements.

   - <mark>i</mark> to iterate through the vector.

- If the current element is different from the last unique element, it's placed at <mark>unique_idx + 1</mark>.

- The function returns the new length <mark>(unique_idx + 1)</mark>.

### <mark>main</mark> Function:
#### - Initializes a vector with duplicates.

Calls remove_duplicates() to modify the vector in place.

Prints the unique elements using slicing (&nums[..new_len]).
