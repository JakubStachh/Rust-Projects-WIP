
# 📌 Intersection of Two Arrays in Rust

## 🚀 Description
This Rust program finds the intersection of `two arrays (vectors)`. The intersection consists of the common elements that appear in both arrays. The result is a vector containing the unique elements from both arrays that exist in both.

The algorithm uses HashSets to efficiently determine the intersection of the two input arrays. By converting both arrays into sets, the program ensures that each element is unique in the final result and performs the intersection operation efficiently.

## 🔍 How It Works
### The function intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32>:

- Converts both input vectors (nums1 and nums2) into HashSets.

- The intersection() method is then called on one set to find common elements with the other set.

- Finally, it collects the result into a vector of integers.

### Key Points:
- HashSet is used to eliminate duplicate elements and to perform set operations efficiently.

- Intersection operation finds the common elements between the two sets.

- cloned() is used to ensure that the values in the intersection are owned and can be collected into a vector.

#### The main function demonstrates how to use intersection() to find the common elements between two vectors.

## 🎯 Example Output
```sh
Intersection: [2]
```

## 📂 Explanation
### intersection Function:
- Input: Two vectors (nums1 and nums2) containing integers.

- Output: A vector containing the intersection (common elements) between the two input vectors.

- #### Logic:

     - The input vectors are converted into HashSets, ensuring that each element appears only once in each set.

     - The intersection method of the HashSet is used to find the common elements between the two sets.

     - The result is then collected into a vector and returned.

### main Function:
- The main function creates two vectors nums1 and nums2, calls the intersection() function, and prints the result.
