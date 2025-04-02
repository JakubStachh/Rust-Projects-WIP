# 📌 Fibonacci Sequence in Rust

## 🚀 Description
This Rust program computes the n-th Fibonacci number using dynamic programming. The Fibonacci sequence is a series of numbers in which each number is the sum of the two preceding ones, usually starting with 0 and 1.

The program efficiently computes the Fibonacci number by storing intermediate results in a vector to avoid redundant calculations, making it optimal in terms of time complexity.

## 🔍 How It Works
### The function fibonacci(n: usize) -> usize:

- Takes an integer n as input, which represents the position in the Fibonacci sequence.

- Initializes a vector fib of size n+1 to store the Fibonacci numbers up to n.

- The first two numbers in the Fibonacci sequence are set as 0 and 1 respectively.

- A loop is then used to fill the vector with Fibonacci numbers by iterating from 2 to n and using the recurrence relation fib[i] = fib[i - 1] + fib[i - 2].

- The function returns the Fibonacci number at position n.

### The main function demonstrates how to use the fibonacci() function to calculate and display the Fibonacci number for n = 10.

## 🎯 Example Output
```sh
Fibonacci of 10: 55
```

## 📂 Explanation

### fibonacci Function:
- Input: An integer n representing the position in the Fibonacci sequence.

- Output: The n-th Fibonacci number.

- #### Logic:

   - Initializes a vector fib with the base case values fib[0] = 0 and fib[1] = 1.

   - Iterates from index 2 to n, calculating Fibonacci numbers using the recurrence relation.

   - Returns the Fibonacci number at position n.

### main Function:
- The main function calls the fibonacci() function to calculate the Fibonacci number for n = 10 and prints the result.

