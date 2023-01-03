use std::collections::HashMap;

fn count_chars(distinct_chars: usize) -> usize {
    let input = include_str!("../../input/day6");
    let mut start = 0;
    let mut end = distinct_chars;
    let mut seen = HashMap::new();
    for i in 0..distinct_chars {
        let idx = i;
        let ch = input.chars().nth(idx).unwrap();
        *seen.entry(ch).or_insert(0) += 1;
    }
    while seen.len() != distinct_chars {
        let ch = input.chars().nth(start).unwrap();
        if *seen.get(&ch).unwrap() == 1 {
            seen.remove(&ch);
        } else {
            *seen.entry(ch).or_default() -= 1;
        }
        let new_ch = input.chars().nth(end).unwrap();
        *seen.entry(new_ch).or_insert(0) += 1;
        start += 1;
        end += 1;
    }
    end
}

fn main() {
    let p1 = count_chars(4);
    let p2 = count_chars(14);
    println!("p1: {p1}");
    println!("p2: {p2}");
}
