# 📌 Breadth-First Search (BFS) in Rust

## 🚀 Description
This Rust program implements Breadth-First Search (BFS) for traversing a graph. BFS is an algorithm used to explore the nodes and edges of a graph in breadth-first order, meaning it visits all the nodes at the present depth level before moving on to nodes at the next depth level.

## 🔍 How It Works
###The function bfs(graph: &HashMap<i32, Vec<i32>>, start: i32):

- Takes a graph represented as a HashMap<i32, Vec<i32>>, where each key is a node, and its associated value is a vector of neighboring nodes.

- The start parameter represents the node to start the BFS traversal from.

- Initializes a visited HashMap to keep track of which nodes have been visited.

- Uses a VecDeque (a double-ended queue) to store nodes to be explored next.

- For each node, it prints the node's value and adds its unvisited neighbors to the queue.

- The BFS continues until all reachable nodes are visited.

- The main function builds a sample graph, and then calls the bfs() function to perform the traversal starting from a given node.

## 🎯 Example Output

```sh
BFS Traversal:
0
1
2
3
4
5
```

## 📂 Explanation

### bfs Function:
- Implements the breadth-first search traversal.

- Uses a queue (VecDeque) to maintain nodes to visit.

- Marks nodes as visited and explores their unvisited neighbors.

### main Function:
- Creates a sample graph represented by a HashMap.

- The graph is undirected, and each node is connected to its neighbors.

```sh
    0 - 1 - 3
    |   |
    2 - 4
    |
    5
```

- Calls the bfs() function starting from node 0.
