fn fibonacci(n: usize) -> usize {
    let mut fib = vec![0; n + 1];
    fib[1] = 1;
    
    for i in 2..=n {
        fib[i] = fib[i - 1] + fib[i - 2];
    }
    
    fib[n]
}

fn main() {
    println!("Fibonacci of 10: {}", fibonacci(10)); // Output: 55
}
