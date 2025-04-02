# 📌 Breadth-First Search (BFS) in Rust

## 🚀 Description

This Rust program demonstrates the Breadth-First Search (BFS) algorithm using a graph
represented as an adjacency list with **HashMap<i32**, **Vec<i32>>**. 
The BFS traversal is implemented using VecDeque for efficient queue operations.

## 🔍 How It Works

The bfs function performs a BFS traversal from a given starting node.

A HashMap is used to track visited nodes.

A VecDeque is used as a queue to process nodes in FIFO order.

The graph is stored as an adjacency list, where each node maps to a list of its neighbors.

The traversal order is printed to the console.
