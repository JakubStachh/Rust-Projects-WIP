
# 📌 Longest Substring Without Repeating Characters in Rust

## 🚀 Description
This Rust program determines the length of the longest substring without repeating characters in a given string. The algorithm uses a sliding window approach, combined with a HashMap to track the most recent indices of characters, enabling an efficient way to calculate the longest substring without repeating characters.

## 🔍 How It Works
### The function length_of_longest_substring(s: &str) -> usize:

- Sliding Window: It maintains a sliding window using two pointers: left and right. The right pointer expands the window by moving through the string, while the left pointer ensures that the window does not contain any repeating characters.

- HashMap: A HashMap is used to store the most recent index of each character in the string.

- When a character is encountered that has already been seen within the current window, the left pointer is moved to the right of the last occurrence of that character.

- The maximum length of the window is tracked as the right pointer moves through the string.

### Key Points:
- HashMap stores the index of the last occurrence of each character.

- Sliding Window technique helps in tracking the longest substring without repeating characters.

- Efficiency: The algorithm runs in linear time O(n), where n is the length of the string.

#### The main function demonstrates how to use length_of_longest_substring() to find the longest substring length in a string.

## 🎯 Example Output
```sh
Longest substring length: 3
```

## 📂 Explanation
### length_of_longest_substring Function:
- Input: A string s.

- Output: The length of the longest substring without repeating characters.

- #### Logic:

     - The function iterates through the string using the right pointer.

     - For each character, it checks if it has appeared before (by checking the HashMap).

     - If it has appeared, the left pointer is moved to the position right after the last occurrence of the character.

     - The function tracks the maximum length of the window where all characters are unique.

### main Function:
- The main function creates a string s, calls the length_of_longest_substring() function, and prints the result.
