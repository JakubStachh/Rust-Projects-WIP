# 📌 Merge Intervals in Rust

## 🚀 Description
This Rust program implements the **merge intervals algorithm**. It takes a collection of intervals and merges any overlapping intervals into a single interval. 
The goal is to return a list of intervals where no two intervals overlap.

## 🔍 How It Works
### Sorting:

- The intervals are sorted by their **start time** to make merging easier.

### Merging:

- We iterate over the sorted intervals and merge overlapping ones.

- If an interval overlaps with the last interval in the merged list, we merge them by updating the end time of the last merged interval.

- If an interval does not overlap, we add it to the merged list.

### Edge Case:

- If the input list of intervals is empty, we return an empty list.

## Key Points:
- **Sorting**: We sort the intervals to ensure that we can easily compare adjacent intervals.

- **Cloning**: The <mark>clone()</mark> method is used to create a copy of intervals to avoid ownership issues while merging.

- <mark>max()</mark>: The <mark>max()</mark> function from <mark>std::cmp</mark> is used to determine the larger of two end times during merging.

## 🎯 Example Output
```sh
[Interval { start: 1, end: 6 }, Interval { start: 8, end: 10 }, Interval { start: 15, end: 18 }]
```

## 📂 Explanation
### <mark>Interval</mark> Struct:
- The <mark>Interval</mark> struct holds two fields: <mark>start</mark> and <mark>end</mark>, which represent the start and end times of an interval.

### <mark>merge_intervals</mark> Function:
- #### Sort Intervals:

   - First, the intervals are sorted based on their start times using <mark>sort_by</mark> and <mark>cmp</mark>.

- #### Iterate and Merge:

   - We iterate through the sorted intervals, checking for overlaps. If two intervals overlap, we merge them by updating the end time of the last merged interval.

   - If there is no overlap, we simply add the current interval to the merged list.

- #### Return: The function returns the list of merged intervals.

## <mark>main</mark> Function:
- A sample set of intervals is provided in <mark>main</mark>.

- The function <mark>merge_intervals</mark> is called to merge the intervals, and the result is printed.
