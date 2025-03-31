use std::collections::{VecDeque, HashMap};

fn bfs(graph: &HashMap<i32, Vec<i32>>, start: i32) {
    let mut visited = HashMap::new();
    let mut queue = VecDeque::new();
    
    queue.push_back(start);
    visited.insert(start, true);

    while let Some(node) = queue.pop_front() {
        println!("{}", node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if visited.get(&neighbor).is_none() {
                    visited.insert(neighbor, true);
                    queue.push_back(neighbor);
                }
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

    println!("BFS Traversal:");
    bfs(&graph, 0); // Output: 0 1 2 3 4 5
}
