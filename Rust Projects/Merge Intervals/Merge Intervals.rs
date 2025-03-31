use std::cmp::max;

#[derive(Debug)]
struct Interval {
    start: i32,
    end: i32,
}

fn merge_intervals(mut intervals: Vec<Interval>) -> Vec<Interval> {
    if intervals.is_empty() {
        return vec![];
    }

    intervals.sort_by(|a, b| a.start.cmp(&b.start));

    let mut merged = vec![intervals[0].clone()];

    for i in 1..intervals.len() {
        let last = merged.last_mut().unwrap();
        if intervals[i].start <= last.end {
            last.end = max(last.end, intervals[i].end);
        } else {
            merged.push(intervals[i].clone());
        }
    }

    merged
}

fn main() {
    let intervals = vec![
        Interval { start: 1, end: 3 },
        Interval { start: 2, end: 6 },
        Interval { start: 8, end: 10 },
        Interval { start: 15, end: 18 },
    ];

    let merged = merge_intervals(intervals);
    println!("{:?}", merged);  // Output: [Interval { start: 1, end: 6 }, Interval { start: 8, end: 10 }, Interval { start: 15, end: 18 }]
}
