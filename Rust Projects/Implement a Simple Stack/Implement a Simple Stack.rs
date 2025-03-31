struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }

    fn push(&mut self, item: T) {
        self.items.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
}

fn main() {
    let mut stack = Stack::new();
    stack.push(10);
    stack.push(20);
    stack.push(30);

    println!("Top element: {:?}", stack.peek()); // Output: Some(30)
    println!("Popped element: {:?}", stack.pop()); // Output: Some(30)
    println!("Top element after pop: {:?}", stack.peek()); // Output: Some(20)
}
