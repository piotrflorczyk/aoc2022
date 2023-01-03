use std::collections::HashMap;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Clone, Debug)]
struct Move {
    checks: Vec<(i32, i32)>,
    mov: (i32, i32),
}

fn get_first_available_move(elf: &Point, grid: &[Vec<char>], moves: &VecDeque<Move>) -> Point {
    let (elfx, elfy) = (elf.x, elf.y);
    let all_dirs = vec![
        (-1, -1),
        (-1, 1),
        (-1, 0),
        (0, 1),
        (0, -1),
        (1, 0),
        (1, -1),
        (1, 1),
    ];
    if all_dirs
        .iter()
        .all(|(dx, dy)| grid[(dy + elfy as i32) as usize][(dx + elfx as i32) as usize] == '.')
    {
        return *elf;
    }
    for mov in moves {
        if mov.checks.iter().all(|(chkx, chky)| {
            grid[(chky + elfy as i32) as usize][(chkx + elfx as i32) as usize] == '.'
        }) {
            return Point {
                x: (elfx as i32 + mov.mov.0) as usize,
                y: (elfy as i32 + mov.mov.1) as usize,
            };
        }
    }
    *elf
}

fn parse_grid(margin: usize) -> Vec<Vec<char>> {
    let line_len = include_str!("../../input/day23")
        .lines()
        .next()
        .unwrap()
        .len();
    let mut grid = Vec::new();
    for _ in 0..margin {
        grid.push(vec!['.'; margin * 2 + line_len]);
    }
    include_str!("../../input/day23").lines().for_each(|line| {
        let mut ln = vec!['.'; margin];
        ln.extend(line.chars());
        ln.extend(vec!['.'; margin].iter());
        grid.push(ln);
    });
    for _ in 0..margin {
        grid.push(vec!['.'; margin * 2 + line_len]);
    }
    grid
}

fn get_elves(grid: &Vec<Vec<char>>) -> Vec<Point> {
    let mut elves = Vec::new();
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                elves.push(Point { y: i, x: j });
            }
        }
    }
    elves
}

fn run_simulation(grid_margin: usize, rounds: usize) -> (Vec<Vec<char>>, Vec<Point>, usize) {
    let mut grid = parse_grid(grid_margin);
    let mut elves = get_elves(&grid);
    let mut moves = VecDeque::from(vec![
        Move {
            checks: vec![(-1, -1), (0, -1), (1, -1)],
            mov: (0, -1),
        },
        Move {
            checks: vec![(-1, 1), (0, 1), (1, 1)],
            mov: (0, 1),
        },
        Move {
            checks: vec![(-1, -1), (-1, 0), (-1, 1)],
            mov: (-1, 0),
        },
        Move {
            checks: vec![(1, -1), (1, 0), (1, 1)],
            mov: (1, 0),
        },
    ]);

    for round in 0..rounds {
        let mut move_map: HashMap<Point, Option<&Point>> = HashMap::new();
        for elf in &elves {
            let elf_move = get_first_available_move(elf, &grid, &moves);
            if move_map.contains_key(&elf_move) {
                move_map.insert(*elf, Some(elf));
                if move_map[&elf_move].is_some() {
                    let elf_val = move_map[&elf_move].unwrap();
                    *move_map.get_mut(&elf_move).unwrap() = None;
                    move_map.insert(*elf_val, Some(elf_val));
                }
            } else {
                move_map.insert(elf_move, Some(elf));
            }
        }
        // remove old elves
        for elf in &elves {
            grid[elf.y][elf.x] = '.';
        }

        // if there is no effective move to do
        if move_map
            .iter()
            .all(|(to, from)| from.is_none() || to == from.unwrap())
        {
            return (grid, elves, round + 1);
        }

        // perform elves moves
        elves = move_map
            .iter()
            .filter(|(_, from)| from.is_some())
            .map(|(to, _)| *to)
            .collect::<Vec<_>>();

        // render new elves
        for elf in &elves {
            grid[elf.y][elf.x] = '#';
        }

        // shift moves
        let mov = moves.pop_front().unwrap();
        moves.push_back(mov);
    }
    (grid, elves, rounds)
}

fn main() {
    let (grid, elves, _) = run_simulation(10, 10);
    let min_x = elves.iter().min_by_key(|e| e.x).unwrap().x;
    let max_x = elves.iter().max_by_key(|e| e.x).unwrap().x;
    let min_y = elves.iter().min_by_key(|e| e.y).unwrap().y;
    let max_y = elves.iter().max_by_key(|e| e.y).unwrap().y;

    let mut empty = 0;
    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            if grid[y][x] == '.' {
                empty += 1;
            }
        }
    }
    println!("p1: {empty}");

    let (_, _, rounds) = run_simulation(100, 10000);
    println!("p2: {rounds}");
}
