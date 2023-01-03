use std::collections::HashSet;

fn char_to_priority(char: u8) -> usize {
    if char >= b'a' {
        usize::from(char - b'a')
    } else {
        usize::from(char - b'A' + 26)
    }
}
fn priorities_p1() -> usize {
    include_str!("../../input/day3")
        .lines()
        .map(|line| {
            let half_point = line.len() / 2;
            let (left, right) = line.split_at(half_point);
            let found_chars: HashSet<&u8> = HashSet::from_iter(left.as_bytes());
            let repeated_ch = right.bytes().find(|ch| found_chars.contains(ch));
            char_to_priority(repeated_ch.unwrap()) + 1
        })
        .sum::<usize>()
}

fn priorities_p2() -> usize {
    let mut iter = 0;
    let mut chars = [true; 52];
    include_str!("../../input/day3")
        .lines()
        .map(|line| {
            let mut result = 0;
            chars = if iter == 2 {
                let ch = line.bytes().find(|ch| chars[char_to_priority(*ch)]);
                result = char_to_priority(ch.unwrap()) + 1;
                [true; 52]
            } else {
                let mut line_chars: [bool; 52] = [false; 52];
                line.bytes().for_each(|ch| {
                    line_chars[char_to_priority(ch)] = chars[char_to_priority(ch)];
                });
                line_chars
            };
            iter = (iter + 1) % 3;
            result
        })
        .sum::<usize>()
}

fn main() {
    let p1 = priorities_p1();
    println!("p1: {p1}");
    let p2 = priorities_p2();
    println!("p2: {p2}");
}
