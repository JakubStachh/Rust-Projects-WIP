# 📌 Anagram Checker in Rust

## 🚀 Description

This Rust program checks whether two given strings are anagrams of each other. Two words are considered anagrams if they contain the same characters in the same frequencies, but in any order.

## 🔍 How It Works

The function are_anagrams(s1: &str, s2: &str) -> bool:

Uses two HashMaps to store character counts for both input strings.

Iterates over each character in both strings, updating their respective frequency maps.

Compares both maps to determine if the words are anagrams.

The main function tests the logic with example word pairs.

