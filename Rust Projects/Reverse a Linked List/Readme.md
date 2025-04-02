# 📌 Reverse a Singly Linked List in Rust

## 🚀 Description
This Rust program **reverses a singly linked list** using an iterative approach. 
The linked list is built usin `Option<Box<Node>>`, where each node contains a value and a reference to the next node.

🔍 How It Works
Key Steps:
Define the Node Struct:

Each node has an i32 value and an Option<Box<Node>> for the next pointer.

Uses #[derive(Debug, Clone)] to enable cloning and debugging.

Reverse Function (reverse_linked_list):

Uses three pointers:

prev → Keeps track of the reversed list.

current → Iterates through the original list.

next → Temporarily stores the next node.

Iterates through the list, reversing the next pointers.

Print Function (print_list):

Iterates through the linked list and prints its values.

Main Function:

Creates a linked list: 1 -> 2 -> 3 -> None

Prints the original and reversed lists.

🎯 Example Output
sh
Copy
Edit
Original List:
1 -> 2 -> 3 -> None
Reversed List:
3 -> 2 -> 1 -> None
📂 Explanation
reverse_linked_list Function:
Uses three pointers to reverse the list in O(n) time.

current iterates through the list.

prev keeps track of the reversed list.

node.next.take() moves ownership of the next node.

print_list Function:
Traverses the list and prints the values.

main Function:
Creates nodes and links them together.

Clones the list before reversing it to avoid ownership issues.

