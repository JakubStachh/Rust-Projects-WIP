# 📌 Anagram Checker in Rust

## 🚀 Description
This Rust program checks whether two given strings are anagrams of each other. Two words are considered anagrams if they contain the same characters in the same frequencies, but in any order.

## 🔍 How It Works
### The function `are_anagrams(s1: &str, s2: &str) -> bool`:

- Uses two **HashMaps** to store character counts for both input strings.

- Iterates over each character in both strings, updating their respective frequency maps.

- Compares both maps to determine if the words are anagrams.

- If both maps are equal, the function returns true, meaning the words are anagrams.

- If the maps are not equal, the function returns false.

### The main function demonstrates how to use the `are_anagrams()` function with two test cases: one where the words are anagrams and one where they are not.

## 🎯 Example Output
```sh
true
false
```

## 📂 Explanation
### `are_anagrams Function`:
- This function takes two strings as inputs and creates two HashMaps to count the frequency of each character in the strings.

- It iterates over the characters of both strings and updates their respective maps.

- Finally, it checks whether the two maps are equal.

### `main Function`:
- The `main Function` tests the `are_anagrams()` function with the following test cases:

- **"listen"** and **"silent"** (anagrams).

- **"hello"** and **"world"** (not anagrams).
