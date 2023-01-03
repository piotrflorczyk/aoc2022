use std::collections::HashMap;

fn snafu_to_dec(num: &str, mapping: &HashMap<char, i64>) -> i64 {
    num.chars()
        .map(|ch| mapping[&ch])
        .fold(0, |acc, val| acc * 5 + val)
}

fn dec_to_snafu(dec_num: i64, mapping: &HashMap<i64, char>) -> String {
    let mut snafu = String::new();
    let mut num = dec_num;
    while num > 0 {
        let rem = num % 5;
        snafu.push(mapping[&rem]);
        num /= 5;
        if rem > 2 {
            num += 1;
        }
    }
    snafu.chars().rev().collect::<String>()
}

fn main() {
    let snafu_to_dec_map = HashMap::from([('0', 0), ('1', 1), ('2', 2), ('-', -1), ('=', -2)]);
    let dec_to_snafu_map = HashMap::from([(0, '0'), (1, '1'), (2, '2'), (3, '='), (4, '-')]);

    let numbers = include_str!("../../input/day25")
        .lines()
        .map(|line| snafu_to_dec(line, &snafu_to_dec_map))
        .collect::<Vec<_>>();

    let sum = numbers.iter().sum::<i64>();
    let snafu_sum = dec_to_snafu(sum, &dec_to_snafu_map);

    println!("Sum decimal: {sum:?}");
    println!("Sum SNAFU (p1): {snafu_sum:?}");
}
