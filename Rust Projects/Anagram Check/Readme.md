#README

##Anagram Checker in Rust
This Rust program determines whether two given strings are anagrams of each other. Two words are considered anagrams if they contain the same characters in the same frequencies, but in any order.

###How It Works
The function are_anagrams(s1: &str, s2: &str) -> bool:

Uses a HashMap to count the occurrences of each character in both input strings.

Compares the frequency maps to determine if the words are anagrams.

The main function tests this logic with example words.
