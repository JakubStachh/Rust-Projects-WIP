# 📌 Palindrome Checker in Rust
## 🚀 Description
This Rust program checks whether a given string is a **palindrome**. 
A palindrome is a word, phrase, or sequence that reads the same forward and backward (e.g., "`racecar`" is a palindrome, while "`hello`" is not).

## 🔍 How It Works
### Key Steps:
#### 1. Convert the String to a Character Vector (`Vec<char>`):

   - The input string is converted into a vector of characters for easy index-based access.

#### 2. Compare Characters:

   - The function iterates from the start of the string to the midpoint, comparing characters at symmetric positions.

#### 3. Return Result:

   - If a mismatch is found, it returns `false`.

   - If all character pairs match, the function returns `true`, confirming a palindrome.

## 🎯 Example Output
```sh
true
false
```

## 📂 Explanation
### `is_palindrome` Function:
- Converts the input string into a vector of characters.

- Iterates through the first half of the string and compares each character with its corresponding character from the end.

- If any mismatch is found, the function returns `false`; otherwise, it returns `true`.

### `main` Function:
- Calls `is_palindrome` with "`racecar`" (should return `true`).

- Calls `is_palindrome` with "`hello`" (should return `false`).

- Prints the results.
