#📌 Depth-First Search (DFS) in Rust

##🚀 Description

This Rust program performs a Depth-First Search (DFS) traversal on an undirected graph using recursion. DFS is a graph traversal algorithm that explores as far as possible along a branch before backtracking.

##🔍 How It Works

**The function dfs(graph: &HashMap<i32, Vec<i32>>, node: i32, visited: &mut HashMap<i32, bool>):**

Prints the current node.

Marks the node as visited in a HashMap.

Recursively visits all unvisited neighbors.

**The main function:**

Initializes a graph using HashMap<i32, Vec<i32>>.

Calls dfs() to start traversal from node 0.
