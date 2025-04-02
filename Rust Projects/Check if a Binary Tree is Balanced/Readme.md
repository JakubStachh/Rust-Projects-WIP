
# 📌 Balanced Binary Tree in Rust

## 🚀 Description

This Rust program checks whether a binary tree is height-balanced. A binary tree is considered balanced if, for every node, the difference in height between its left and right subtrees is at most 1.

## 🔍 How It Works

**The function is_balanced(root: Option<Box<TreeNode>>) -> bool:**

Uses a helper function height(node: Option<&Box<TreeNode>>) -> i32 to calculate the height of the tree recursively.

If any subtree is unbalanced, the function returns -1.

Otherwise, it returns the correct height of the subtree.

The tree is balanced if the height function never returns -1.

The main function builds a sample binary tree and calls is_balanced() to check if it is balanced.
