use regex::Regex;
use std::cmp::max;

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Pair {
    sensor: Point,
    beacon: Point,
}

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

fn parse_input() -> Vec<Pair> {
    let lines = include_str!("../../input/day15").lines();
    let re = Regex::new(r"Sensor at x=([^,]+), y=([^:]+): closest beacon is at x=([^,]+), y=(.+)")
        .unwrap();
    lines
        .map(|line| {
            let cap = re.captures_iter(line).next().unwrap();
            Pair {
                sensor: Point {
                    x: cap[1].parse::<i32>().unwrap(),
                    y: cap[2].parse::<i32>().unwrap(),
                },
                beacon: Point {
                    x: cap[3].parse::<i32>().unwrap(),
                    y: cap[4].parse::<i32>().unwrap(),
                },
            }
        })
        .collect::<Vec<_>>()
}

fn get_ranges(pairs: &[Pair], row: i32) -> Vec<Range> {
    let mut ranges = pairs
        .iter()
        .map(|pair| {
            let distance =
                (pair.beacon.x - pair.sensor.x).abs() + (pair.beacon.y - pair.sensor.y).abs();
            if (pair.sensor.y - row).abs() <= distance {
                let length = distance - (pair.sensor.y - row).abs();
                Some(Range {
                    start: pair.sensor.x - length,
                    end: pair.sensor.x + length,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect::<Vec<_>>();
    ranges.sort_by_key(|x| x.start);
    ranges
}

fn day15_1() {
    let pairs = parse_input();
    let ranges = get_ranges(&pairs, 2000000);

    let mut max_x = ranges[0].end;
    let mut gapes = 0;
    for i in 1..ranges.len() {
        if ranges[i].start > max_x {
            gapes += ranges[i].start - max_x - 1
        }
        max_x = max(max_x, ranges[i].end);
    }
    let no_beacon = max_x - ranges[0].start - gapes;
    println!("Number of beacons p1: {no_beacon:?}");
}

fn day15_2() {
    let pairs = parse_input();
    for y in 0..4000000 {
        let ranges = get_ranges(&pairs, y);
        let mut max_x = ranges[0].end;
        for i in 1..ranges.len() {
            if ranges[i].start > max_x {
                let tuning_freq = (max_x + 1) as i128 * 4000000 + y as i128;
                println!("Tuning frequency p2: {tuning_freq}");
                return;
            }
            max_x = max(max_x, ranges[i].end);
        }
    }
}

fn main() {
    day15_1();
    day15_2();
}
