# 📌 Stack Implementation in Rust Using Generics

## 🚀 Description
This Rust program implements a generic stack using a <mark>Vec<T></mark>. The <mark>Stack</mark> struct supports basic stack operations, such as <mark>push</mark>, <mark>pop</mark>, and <mark>peek</mark>. 
The stack is implemented with generics, meaning it can store elements of any type <mark>T</mark>.

## 🔍 How It Works
### The <mark>Stack<T></mark> struct:

- <mark>items</mark>: A vector that holds the stack elements.

### <mark>Operations</mark>:
- <mark>push(item: T)</mark>: Adds an item to the top of the stack.

- <mark>pop()>/mark>: Removes and returns the top item from the stack. If the stack is empty, it returns <mark>None</mark>.

- <mark>peek()</mark>: Returns a reference to the top item without removing it. If the stack is empty, it returns <mark>None</mark>.

#### The <mark>Stack</mark> is implemented using generics, which allows it to work with any type.

## 🎯 Example Output
```sh
Top element: Some(30)
Popped element: Some(30)
Top element after pop: Some(20)
```

## 📂 Explanation
### <mark>Stack<T></mark> Struct:
- <mark>items</mark>: The vector that holds the stack's elements. It's used as the underlying container for the stack.

### <mark>push</mark> Method:
- Adds an item to the stack by pushing it onto the <mark>items</mark> vector.

### <mark>pop</mark> Method:
- Removes the top item from the stack by calling <mark>pop()</mark> on the <mark>items</mark> vector. If the vector is empty, it returns <mark>None</mark>.

### <mark>peek</mark> Method:
- Returns a reference to the last element of the <mark>items</mark> vector, which is the top of the stack. If the stack is empty, it returns <mark>None</mark>.

### Generics:
- The stack is generic (<mark>Stack<T></mark>), so it can hold elements of any type <mark>T</mark>. This allows the stack to be reused for different types of data.
