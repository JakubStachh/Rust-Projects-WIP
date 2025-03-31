use std::collections::HashMap;

fn dfs(graph: &HashMap<i32, Vec<i32>>, node: i32, visited: &mut HashMap<i32, bool>) {
    println!("{}", node);
    visited.insert(node, true);

    if let Some(neighbors) = graph.get(&node) {
        for &neighbor in neighbors {
            if visited.get(&neighbor).is_none() {
                dfs(graph, neighbor, visited);
            }
        }
    }
}

fn main() {
    let mut graph = HashMap::new();
    graph.insert(0, vec![1, 2]);
    graph.insert(1, vec![0, 3, 4]);
    graph.insert(2, vec![0, 5]);
    graph.insert(3, vec![1]);
    graph.insert(4, vec![1]);
    graph.insert(5, vec![2]);

    let mut visited = HashMap::new();
    println!("DFS Traversal:");
    dfs(&graph, 0, &mut visited); // Output: 0 1 3 4 2 5
}
