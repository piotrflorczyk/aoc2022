use std::collections::HashSet;

fn is_touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    let (headx, heady) = head;
    let (tailx, taily) = tail;
    tailx >= (headx - 1) && tailx <= (headx + 1) && taily >= (heady - 1) && taily <= (heady + 1)
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let (headx, heady) = head;
    let (tailx, taily) = tail;
    if headx == tailx {
        (tailx, if taily > heady { taily - 1 } else { taily + 1 })
    } else if heady == taily {
        (if tailx > headx { tailx - 1 } else { tailx + 1 }, taily)
    } else {
        (
            if tailx > headx { tailx - 1 } else { tailx + 1 },
            if taily > heady { taily - 1 } else { taily + 1 },
        )
    }
}

fn simulate_rope(len: usize) -> usize {
    let lines = include_str!("../../input/day9").lines();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    let mut ropes = vec![(0, 0); len];
    visited.insert((0, 0));

    lines.for_each(|line| {
        let (dir_s, rep_s) = line.split_once(' ').unwrap();
        let ((x, y), rep) = (
            match dir_s {
                "R" => (1, 0),
                "L" => (-1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!(),
            },
            rep_s.parse::<i32>().unwrap(),
        );
        for _ in 0..rep {
            ropes[0].0 += x;
            ropes[0].1 += y;
            for i in 1..ropes.len() {
                if !is_touching(ropes[i - 1], ropes[i]) {
                    ropes[i] = move_tail(ropes[i - 1], ropes[i]);
                } else {
                    break;
                }
            }
            visited.insert(ropes[ropes.len() - 1]);
        }
    });
    visited.len()
}

fn main() {
    let p1 = simulate_rope(2);
    let p2 = simulate_rope(10);

    println!("p1: {p1}");
    println!("p2: {p2}");
}
