# 📌 Implementing a Queue using Two Stacks in Rust

## 🚀 Description
This Rust program implements a **queue** using two **stacks**. The queue supports the basic operations:

- `push(x)` — Pushes an element to the back of the queue.

- `pop()` — Removes the element from the front of the queue.

- `peek()` — Retrieves the front element without removing it.

- `empty()` — Checks whether the queue is empty.

By using two stacks, the program simulates the behavior of a queue, where elements are pushed into one stack (`stack_in`) and popped from the other stack (`stack_out`). 
When `stack_out` is empty, elements from `stack_in` are moved to `stack_out`, reversing their order.

## 🔍 How It Works
### The `MyQueue` struct:

- `stack_in`: A stack where elements are pushed.

- `stack_out`: A stack from which elements are popped (after transferring elements from `stack_in` if necessary).

### Operations:
- `push(x: i32)`: Adds an element to the `stack_in`.

- `pop()`: If `stack_out` is empty, it moves all elements from `stack_in` to `stack_out`. Then, it pops from <makr>stack_out`.

- `peek()`: If `stack_out` is empty, it returns the first element from `stack_in`. Otherwise, it returns the last element from `stack_out`.

- <mark>empty()</mark>: Returns <mark>true</mark> if both stacks are empty, otherwise returns `false`.

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

- <mark>stack_out</mark>: A vector that serves as the "output" stack where elements are popped. If <mark>stack_out</mark> is empty, elements are moved from <mark>stack_in</mark> to <mark>stack_out</mark>.

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
#
