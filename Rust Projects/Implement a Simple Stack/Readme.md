# 📌 Stack Implementation in Rust Using Generics

## 🚀 Description
This Rust program implements a generic stack using a `Vec<T>`. The `Stack` struct supports basic stack operations, such as `push`, `pop`, and `peek`. 
The stack is implemented with generics, meaning it can store elements of any type `T`.

## 🔍 How It Works
### The `Stack<T>` struct:

- `items`: A vector that holds the stack elements.

### `Operations`:
- `push(item: T)`: Adds an item to the top of the stack.

- `pop()`: Removes and returns the top item from the stack. If the stack is empty, it returns `None`.

- `peek()`: Returns a reference to the top item without removing it. If the stack is empty, it returns `None`.

#### The `Stack` is implemented using generics, which allows it to work with any type.

## 🎯 Example Output
```sh
Top element: Some(30)
Popped element: Some(30)
Top element after pop: Some(20)
```

## 📂 Explanation
### `Stack<T>` Struct:
- `items`: The vector that holds the stack's elements. It's used as the underlying container for the stack.

### `push` Method:
- Adds an item to the stack by pushing it onto the `items` vector.

### `pop` Method:
- Removes the top item from the stack by calling `pop()` on the `items` vector. If the vector is empty, it returns `None`.

### `peek` Method:
- Returns a reference to the last element of the `items` vector, which is the top of the stack. If the stack is empty, it returns `None`.

### Generics:
- The stack is generic (`Stack<T>`), so it can hold elements of any type `T`. This allows the stack to be reused for different types of data.
