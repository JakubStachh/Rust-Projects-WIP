
# 📌 Balanced Binary Tree in Rust

## 🚀 Description

This Rust program checks whether a binary tree is height-balanced. A binary tree is considered balanced if, for every node, the difference in height between its left and right subtrees is at most 1.

## 🔍 How It Works

**The function is_balanced(root: Option<Box<TreeNode>>) -> bool:**

- Uses a helper function height(node: Option<&Box<TreeNode>>) -> i32 to calculate the height of the tree recursively.

- If any subtree is unbalanced, the function returns -1.

- Otherwise, it returns the correct height of the subtree.

- The tree is balanced if the height function never returns -1.

- The main function builds a sample binary tree and calls is_balanced() to check if it is balanced.

## 🎯 Example Output
```sh
Visiting node with value: 1
Visiting node with value: 2
Visiting node with value: 4
Visiting node with value: 3
Is tree balanced? true
```
## 📂 Explanation
### TreeNode Struct:

- Represents a binary tree node with a value (val), and optional left and right children.

### **is_balanced** Function:

- Recursively checks the height of left and right subtrees of each node.

- If the height difference between subtrees exceeds 1, it returns -1, indicating the tree is unbalanced.

### height Function:

- Computes the height of the subtree rooted at a node. It checks for balance during recursion.

### main Function:

- Creates a sample tree:
```sh
    1
   / \
  2   3
 /
4
```
- Then, it prints whether the tree is balanced.
