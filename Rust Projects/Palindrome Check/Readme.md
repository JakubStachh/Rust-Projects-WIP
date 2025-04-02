# 📌 Palindrome Checker in Rust
## 🚀 Description
This Rust program checks whether a given string is a **palindrome**. 
A palindrome is a word, phrase, or sequence that reads the same forward and backward (e.g., "<mark>racecar</mark>" is a palindrome, while "<mark>hello</mark>" is not).

## 🔍 How It Works
### Key Steps:
#### 1. Convert the String to a Character Vector (<mark>Vec<char></mark>):

   - The input string is converted into a vector of characters for easy index-based access.

#### 2. Compare Characters:

   - The function iterates from the start of the string to the midpoint, comparing characters at symmetric positions.

#### 3. Return Result:

   - If a mismatch is found, it returns <mark>false</mark>.

   - If all character pairs match, the function returns <mark>true</mark>, confirming a palindrome.

## 🎯 Example Output
```sh
true
false
```

## 📂 Explanation
### <mark>is_palindrome</mark> Function:
- Converts the input string into a vector of characters.

- Iterates through the first half of the string and compares each character with its corresponding character from the end.

- If any mismatch is found, the function returns <mark>false</mark>; otherwise, it returns <mark>true</mark>.

### <mark>main</mark> Function:
- Calls <mark>is_palindrome</mark> with "<mark>racecar</mark>" (should return <mark>true</mark>).

- Calls <mark>is_palindrome</mark> with "<mark>hello</mark>" (should return <mark>false</mark>).

- Prints the results.
