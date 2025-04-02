
# 📌 String Permutation in Rust

## 🚀 Description
This Rust program generates all permutations of a given string using backtracking. It takes a string as input and produces all possible arrangements of its characters. Each permutation is guaranteed to be unique.

The algorithm swaps characters within the string, recursively generating permutations and then backtracks to explore other possibilities.

## 🔍 How It Works
### The function `permute(s: &str) -> Vec<String>`:

- This function takes a string `s` and returns a **vector** containing all permutations of the string.

- It converts the string into a vector of characters and passes it to a helper function (`permute_helper`), which recursively generates all permutations.

- The function `permute_helper`(`s: &mut Vec<char>`, `start: usize, result: &mut Vec<String>`):

- Recursively generates permutations by swapping characters in the **string**.

- When the start **index** reaches the end of the **string**, the current permutation is added to the result.

- The function uses *backtracking** by swapping characters back to their original positions after each recursive call to explore other permutations.

### The main function demonstrates how to use `permute()` to generate permutations of the `string "abc"`.

## 🎯 Example Output
```sh
["abc", "acb", "bac", "bca", "cab", "cba"]
```

## 📂 Explanation
### permute Function:
- Input: A string s for which permutations are to be generated.

- Output: A vector containing all unique permutations of the string.

- #### Logic:

    - Converts the string into **a vector of characters**.

    - Calls the helper function `permute_helper()` to generate permutations.

    - Returns the final vector containing all the permutations.

 ### permute_helper Function:
- Input: A mutable vector of characters, a start index, and a mutable result vector.

- Output: **Modifies** the result vector to include permutations.

- #### Logic:

    - Recursively generates permutations by swapping the start character with all other characters at or after it.

    - When the entire string has been traversed (i.e., `start == s.len()`), the current string is collected as a permutation.

    - The algorithm backtracks by swapping characters back to their original positions.

 ### main Function:
- The main function calls the `permute()` function with the `string "abc"` and prints the generated permutations.
