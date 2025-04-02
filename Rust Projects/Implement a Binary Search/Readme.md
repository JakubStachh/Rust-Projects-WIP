## 🏹 Binary Search in Rust

## 📌 Description

This project implements the Binary Search Algorithm in Rust. Binary Search is an efficient searching algorithm that finds the position of a target value in a sorted array by repeatedly dividing the search interval in half.

## 🚀 How It Works

Start with the left (left = 0) and right (right = arr.len() - 1) pointers.

Find the middle index: mid = (left + right) / 2.

If the middle element is equal to the target, return its index.

If the middle element is smaller than the target, search the right half.

If the middle element is larger than the target, search the left half.

Repeat until the element is found or left > right.
