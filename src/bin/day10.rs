use std::collections::HashSet;

fn day10_1() {
    let lines = include_str!("../../input/day10").lines();
    let sampling_points = HashSet::from([20, 60, 100, 140, 180, 220]);
    let mut x = 1;
    let mut cycle = 1;
    let samples = lines.fold(Vec::new(), |acc, line| {
        let mut res = acc;
        if sampling_points.contains(&cycle) {
            res.push(cycle * x);
        }
        if line.starts_with("noop") {
            cycle += 1;
        } else {
            cycle += 2;
            if sampling_points.contains(&(cycle - 1)) {
                res.push((cycle - 1) * x);
            }
            x += line[5..].parse::<i32>().unwrap();
        }
        res
    });

    let sum = samples.iter().sum::<i32>();
    println!("p1: {sum:?}");
}

fn day10_2() {
    let lines = include_str!("../../input/day10").lines();

    let mut x = 1;
    let mut cycle = 1;
    let output = lines
        .map(|line| {
            let idx = ((cycle - 1) % 40) + 1;
            let mut res = ".".to_string();
            if idx >= (x) && idx <= (x + 2) {
                res = "#".to_string();
            }
            if line.starts_with("noop") {
                cycle += 1;
            } else {
                cycle += 1;
                let idx = ((cycle - 1) % 40) + 1;
                if idx >= (x) && idx <= (x + 2) {
                    res += "#";
                } else {
                    res += ".";
                }
                cycle += 1;
                x += line[5..].parse::<i32>().unwrap();
            }
            res
        })
        .collect::<String>();

    let print_output = output
        .chars()
        .collect::<Vec<char>>()
        .chunks(40)
        .map(|c| c.iter().collect::<String>())
        .collect::<Vec<String>>();

    println!("p2:");
    print_output.iter().for_each(|crt_line| {
        println!("{crt_line}");
    });
}

fn main() {
    day10_1();
    day10_2();
}
