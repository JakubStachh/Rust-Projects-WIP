# 📌 Find Missing Number in Rust

## 🚀 Description
This Rust program finds the missing number in a sequence of numbers from 1 to n. The array arr contains numbers from 1 to n, but one number is missing. The function calculates the missing number by leveraging the arithmetic sum formula.

## 🔍 How It Works
The function find_missing_number(arr: Vec<i32>, n: i32) -> i32:

### It calculates the expected sum of numbers from 1 to n using the formula:

$$
expectedsum = \frac{n \times (n + 1)}{2}
$$

 
Then, it computes the actual sum of the numbers in the array.

The difference between the expected sum and the actual sum gives the missing number.

### Main Function:

- It defines the array arr and the value n, which represents the largest number in the sequence.

- The function find_missing_number is called to find and print the missing number.

## 🎯 Example Output
```sh
The missing number is: 3
```

## 📂 Explanation
### <mark>find_missing_number Function</mark>:
- **Expected Sum**: The sum of numbers from 1 to n is computed using the formula:  
𝑛
×
(
𝑛
+
1
)
2
2
n×(n+1)
​
 .

- **Actual Sum**: The sum of the numbers in the given array arr is calculated.

- Missing Number: The difference between the expected sum and actual sum gives the missing number.

### <mark>Main Function</mark>:
- Defines an array arr containing the sequence with one missing number.

Calls the function find_missing_number to compute and print the missing number.
