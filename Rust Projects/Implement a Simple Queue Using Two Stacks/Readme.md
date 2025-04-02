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

- `empty()`: Returns `true` if both stacks are empty, otherwise returns `false`.

## 🎯 Example Output
```sh
Peek: Some(1)
Peek after pop: Some(2)
Is the queue empty? false
Is the queue empty after pop? true
```

## 📂 Explanation
### `MyQueue` Struct:
- `stack_in`: A vector that serves as the "input" stack where new elements are pushed.

- `stack_out`: A vector that serves as the "output" stack where elements are popped. If `stack_out` is empty, elements are moved from `stack_in` to `stack_out`.

### `push` Method:
- Adds an element to `stack_in`, simulating the enqueue operation of a queue.

### `pop` Method:
- If `stack_out` is empty, it transfers all elements from `stack_in` to `stack_out`, which reverses their order, mimicking the behavior of a queue where the first inserted element is removed first.

- Pops from `stack_out`, returning the front of the queue.

### `peek` Method:
- If `stack_out` is empty, it checks the front of `stack_in`.

- If `stack_out` has elements, it retrieves the front of the queue from `stack_out`.

### `empty` Method:
- Returns `true` if both stacks are empty (indicating the queue is empty), otherwise returns `false`.
#
