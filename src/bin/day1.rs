use std::collections::BinaryHeap;

fn main() {
    let lines = include_str!("../../input/day1")
        .lines()
        .map(|v| v.parse::<i32>().ok())
        .collect::<Vec<_>>();
    let mut heap = BinaryHeap::new();
    lines
        .split(|line| line.is_none())
        .map(|group| group.iter().map(|v| v.unwrap()).sum::<i32>())
        .for_each(|sum| {
            if heap.len() < 3 {
                heap.push(-sum);
            } else if -heap.peek().unwrap() < sum {
                heap.pop();
                heap.push(-sum);
            }
        });
    let top_score = -heap.iter().min().unwrap();
    let top_three = heap.iter().map(|v| -v).sum::<i32>();
    println!("p1: {top_score}");
    println!("p2: {top_three}");
}
