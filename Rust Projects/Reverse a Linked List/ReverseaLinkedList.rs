#[derive(Debug, Clone)]
struct Node {
    value: i32,
    next: Option<Box<Node>>,
}

fn reverse_linked_list(head: Option<Box<Node>>) -> Option<Box<Node>> { // Removed `mut` from `head`
    let mut prev = None;
    let mut current = head;

    while let Some(mut node) = current {
        current = node.next.take();
        node.next = prev;
        prev = Some(node);
    }
    prev
}

fn print_list(mut head: Option<Box<Node>>) {
    while let Some(node) = head {
        print!("{} -> ", node.value);
        head = node.next;
    }
    println!("None");
}

fn main() {
    let node3 = Some(Box::new(Node { value: 3, next: None }));
    let node2 = Some(Box::new(Node { value: 2, next: node3 }));
    let node1 = Some(Box::new(Node { value: 1, next: node2 }));

    println!("Original List:");
    print_list(node1.clone());

    let reversed = reverse_linked_list(node1);
    
    println!("Reversed List:");
    print_list(reversed);
}
