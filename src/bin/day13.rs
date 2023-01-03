use std::cmp::{Ord, Ordering};

#[derive(Debug, Eq)]
enum NestedList {
    Int(u32),
    List(Vec<NestedList>),
}

impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        compare(self, other)
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(compare(self, other))
    }
}

impl PartialEq for NestedList {
    fn eq(&self, other: &Self) -> bool {
        compare(self, other) == Ordering::Equal
    }
}

fn parse_list(data: &str) -> NestedList {
    let mut stack = vec![];
    let mut current_number = None;
    let mut current_list = vec![];
    for ch in data.chars() {
        if ch.is_numeric() {
            current_number = match current_number {
                Some(x) => Some(x * 10 + ch.to_digit(10).unwrap()),
                None => ch.to_digit(10),
            }
        } else if ch == ',' {
            if current_number.is_some() {
                current_list.push(NestedList::Int(current_number.unwrap()));
                current_number = None;
            }
        } else if ch == ']' {
            if current_number.is_some() {
                current_list.push(NestedList::Int(current_number.unwrap()));
                current_number = None;
            }
            let tmp = current_list;
            current_list = stack.pop().unwrap();
            current_list.push(NestedList::List(tmp));
        } else if ch == '[' {
            stack.push(current_list);
            current_list = vec![];
        }
    }
    current_list.pop().unwrap()
}

fn compare(left: &NestedList, right: &NestedList) -> Ordering {
    match (left, right) {
        (NestedList::Int(left_int), NestedList::Int(right_int)) => left_int.cmp(right_int),
        (NestedList::Int(left_int), _) => {
            compare(&NestedList::List(vec![NestedList::Int(*left_int)]), right)
        }
        (_, NestedList::Int(right_int)) => {
            compare(left, &NestedList::List(vec![NestedList::Int(*right_int)]))
        }
        (NestedList::List(left_list), NestedList::List(right_list)) => {
            let mut i = 0;
            while i < left_list.len() && i < right_list.len() {
                let cmp = compare(&left_list[i], &right_list[i]);
                if cmp == Ordering::Equal {
                    i += 1
                } else {
                    return cmp;
                }
            }
            left_list.len().cmp(&right_list.len())
        }
    }
}

fn day13_1() {
    let pairs = include_str!("../../input/day13")
        .split("\n\n")
        .map(|pair| pair.split('\n').map(parse_list).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let results = pairs
        .iter()
        .enumerate()
        .map(|(idx, pair)| match compare(&pair[0], &pair[1]) {
            Ordering::Less => idx + 1,
            _ => 0,
        })
        .sum::<usize>();

    println!("p1: {results:?}");
}

fn day13_2() {
    let mut signals = include_str!("../../input/day13")
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(parse_list)
        .collect::<Vec<_>>();
    signals.push(parse_list("[[2]]"));
    signals.push(parse_list("[[6]]"));
    signals.sort();

    let result = signals
        .iter()
        .enumerate()
        .filter(|(_, signal)| {
            (*signal).eq(&parse_list("[[2]]")) || (*signal).eq(&parse_list("[[6]]"))
        })
        .fold(1, |acc, (idx, _)| acc * (idx + 1));
    println!("p2: {result:?}");
}

fn main() {
    day13_1();
    day13_2();
}
