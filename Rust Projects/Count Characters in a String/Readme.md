
# 📌 Character Frequency Counter in Rust

## 🚀 Description
This Rust program counts how many times each character appears in a given string using a **HashMap**.

## 🔍 How It Works
**The function count_characters(s: &str) -> HashMap<char, i32>:**

- Iterates over each character in the string.

- Uses a HashMap to store character frequencies.

- Increments the count for each character.

**The main function:**

- Defines a sample string ("hello world").

- Calls count_characters to process the string.

- Prints the frequency of each character.


## 🎯 Example Output
```sh
Character 'h' appears 1 times
Character 'e' appears 1 times
Character 'l' appears 3 times
Character 'o' appears 2 times
Character ' ' appears 1 times
Character 'w' appears 1 times
Character 'r' appears 1 times
Character 'd' appears 1 times
```

## 📂 Explanation
- HashMap Usage → The HashMap stores the count for each character.

- Iterating Over the String → The string is iterated using .chars(), which returns each character as char.

- Updating the HashMap → The .entry(ch).or_insert(0) method is used to either insert the character into the map with a count of 0 (if it doesn't exist) or update the existing count.

- Printing the Result → The program prints each character and its count.

