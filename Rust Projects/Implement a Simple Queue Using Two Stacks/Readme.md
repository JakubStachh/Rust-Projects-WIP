# 📌 Implementing a Queue using Two Stacks in Rust

## 🚀 Description
This Rust program implements a **queue** using two **stacks**. The queue supports the basic operations:

- <mark>push(x)</mark> — Pushes an element to the back of the queue.

- <mark>pop()</mark> — Removes the element from the front of the queue.

- <mark>peek()</mark> — Retrieves the front element without removing it.

- <mark>empty()</mark> — Checks whether the queue is empty.

By using two stacks, the program simulates the behavior of a queue, where elements are pushed into one stack (<mark>stack_in</mark>) and popped from the other stack (<mark>stack_out</mark>). 
When <mark>stack_out</mark> is empty, elements from <mark>stack_in</mark> are moved to <mark>stack_out</mark>, reversing their order.

## 🔍 How It Works
### The <mark>MyQueue</mark> struct:

- <mark>stack_in</mark>: A stack where elements are pushed.

- <mark>stack_out</mark>: A stack from which elements are popped (after transferring elements from <mark>stack_in</mark> if necessary).

### Operations:
- <mark>push(x: i32)</mark>: Adds an element to the <mark>stack_in</mark>.

- <mark>pop()</mark>: If <mark>stack_out</mark> is empty, it moves all elements from <mark>stack_in</mark> to <mark>stack_out</mark>. Then, it pops from <makr>stack_out</mark>.

- <mark>peek()</mark>: If <mark>stack_out</mark> is empty, it returns the first element from <mark>stack_in</mark>. Otherwise, it returns the last element from <mark>stack_out</mark>.

- <mark>empty()</mark>: Returns <mark>true</mark> if both stacks are empty, otherwise returns <mark>false</mark>.

## 🎯 Example Output
```sh
Peek: Some(1)
Peek after pop: Some(2)
Is the queue empty? false
Is the queue empty after pop? true
```

## 📂 Explanation
### <mark>MyQueue</mark> Struct:
- <mark>stack_in</mark>: A vector that serves as the "input" stack where new elements are pushed.

- <mark>stack_out</mark>: A vector that serves as the "output" stack where elements are popped. If <mark>stack_out<mark> is empty, elements are moved from <mark>stack_in</mark> to <mark>stack_out</mark>.

### <mark>push</mark> Method:
- Adds an element to <mark>stack_in</mark>, simulating the enqueue operation of a queue.

### <mark>pop</mark> Method:
- If <mark>stack_out</mark> is empty, it transfers all elements from <mark>stack_in</mark> to <mark>stack_out</mark>, which reverses their order, mimicking the behavior of a queue where the first inserted element is removed first.

- Pops from <mark>stack_out</mark>, returning the front of the queue.

### <mark>peek</mark> Method:
- If <mark>stack_out</mark> is empty, it checks the front of <mark>stack_in</mark>.

- If <mark>stack_out</mark> has elements, it retrieves the front of the queue from <mark>stack_out</mark>.

### <mark>empty</mark> Method:
- Returns <mark>true</mark> if both stacks are empty (indicating the queue is empty), otherwise returns </mark>false<mark>.
