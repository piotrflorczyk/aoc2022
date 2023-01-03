use std::collections::HashMap;

fn day2_p1() {
    let figure_score = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);
    let round_outcome = HashMap::from([
        ("A X", 3),
        ("B Y", 3),
        ("C Z", 3),
        ("A Y", 6),
        ("B Z", 6),
        ("C X", 6),
        ("A Z", 0),
        ("B X", 0),
        ("C Y", 0),
    ]);
    let lines = include_str!("../../input/day2").lines();
    let scores = lines
        .map(|line| {
            let outcome = round_outcome.get(line.trim()).unwrap();
            let play = line.split(' ').nth(1).unwrap();
            let value = figure_score.get(play).unwrap();
            outcome + value
        })
        .sum::<i32>();
    println!("p1 {scores:?}")
}

fn get_move<'a>(opponents_move: &'a str, outcome: &str) -> &'a str {
    match outcome {
        "X" => match opponents_move {
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => unreachable!(),
        },
        "Y" => opponents_move,
        "Z" => match opponents_move {
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

fn day2_p2() {
    let expected_score = HashMap::from([("X", 0), ("Y", 3), ("Z", 6)]);
    let figure_score = HashMap::from([("A", 1), ("B", 2), ("C", 3)]);
    let lines = include_str!("../../input/day2").lines();
    let scores = lines
        .map(|line| {
            let opponents_move = line.split(' ').nth(0).unwrap();
            let expected_outcome = line.split(' ').nth(1).unwrap();
            let my_move = get_move(opponents_move, expected_outcome);
            let move_score = figure_score.get(my_move).unwrap();
            let outcome_score = expected_score.get(expected_outcome).unwrap();
            move_score + outcome_score
        })
        .sum::<i32>();
    println!("p2: {scores:?}")
}

fn main() {
    day2_p1();
    day2_p2();
}
