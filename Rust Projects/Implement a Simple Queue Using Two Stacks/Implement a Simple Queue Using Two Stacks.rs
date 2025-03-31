struct MyQueue {
    stack_in: Vec<i32>,
    stack_out: Vec<i32>,
}

impl MyQueue {
    fn new() -> Self {
        MyQueue {
            stack_in: Vec::new(),
            stack_out: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack_in.push(x);
    }

    fn pop(&mut self) -> Option<i32> {
        if self.stack_out.is_empty() {
            while let Some(val) = self.stack_in.pop() {
                self.stack_out.push(val);
            }
        }
        self.stack_out.pop()
    }

    fn peek(&self) -> Option<&i32> {
        if self.stack_out.is_empty() {
            self.stack_in.first()
        } else {
            self.stack_out.last()
        }
    }

    fn empty(&self) -> bool {
        self.stack_in.is_empty() && self.stack_out.is_empty()
    }
}

fn main() {
    let mut queue = MyQueue::new();
    queue.push(1);
    queue.push(2);
    println!("Peek: {:?}", queue.peek());  // Output: Some(1)
    queue.pop();
    println!("Peek after pop: {:?}", queue.peek());  // Output: Some(2)
}
