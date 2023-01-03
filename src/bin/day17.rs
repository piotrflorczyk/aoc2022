use std::{
    cmp::max,
    collections::HashSet,
    collections::{hash_map::Entry, HashMap},
};

#[derive(Clone, Copy, Debug)]
enum Shape {
    Horizontal,
    Plus,
    ReverseL,
    Vertical,
    Square,
}

fn height(shape: &Shape) -> usize {
    match shape {
        Shape::Horizontal => 1,
        Shape::Plus | Shape::ReverseL => 3,
        Shape::Vertical => 4,
        Shape::Square => 2,
    }
}

fn can_move_right(grid: &[Vec<char>], shape: &Shape, (x, y): (usize, usize)) -> bool {
    match shape {
        Shape::Horizontal => grid[y][x + 4] == '.',
        Shape::Vertical => [0, 1, 2, 3]
            .iter()
            .all(|delta| grid[y - delta][x + 1] == '.'),
        Shape::Square => [0, 1].iter().all(|delta| grid[y - delta][x + 2] == '.'),
        Shape::ReverseL => [0, 1, 2].iter().all(|delta| grid[y - delta][x + 3] == '.'),
        Shape::Plus => {
            grid[y][x + 2] == '.' && grid[y - 1][x + 3] == '.' && grid[y - 2][x + 2] == '.'
        }
    }
}

fn can_move_left(grid: &[Vec<char>], shape: &Shape, (x, y): (usize, usize)) -> bool {
    match shape {
        Shape::Horizontal => grid[y][x - 1] == '.',
        Shape::Vertical => [0, 1, 2, 3]
            .iter()
            .all(|delta| grid[y - delta][x - 1] == '.'),
        Shape::Square => [0, 1]
            .iter()
            .all(|delta| grid[(y - delta) as usize][x - 1] == '.'),
        Shape::ReverseL => {
            grid[y][x + 1] == '.' && grid[y - 1][x + 1] == '.' && grid[y - 2][x - 1] == '.'
        }
        Shape::Plus => grid[y][x] == '.' && grid[y - 1][x - 1] == '.' && grid[y - 2][x] == '.',
    }
}

fn can_move_down(grid: &[Vec<char>], shape: &Shape, (x, y): (usize, usize)) -> bool {
    match shape {
        Shape::Horizontal => [0, 1, 2, 3]
            .iter()
            .all(|delta| grid[y - 1][x + delta] == '.'),
        Shape::Vertical => grid[y - 4][x] == '.',
        Shape::Square => [0, 1].iter().all(|delta| grid[y - 2][x + delta] == '.'),
        Shape::ReverseL => [0, 1, 2].iter().all(|delta| grid[y - 3][x + delta] == '.'),
        Shape::Plus => {
            grid[y - 2][x] == '.' && grid[y - 3][x + 1] == '.' && grid[y - 2][x + 2] == '.'
        }
    }
}

fn render_on_grid(grid: &mut [Vec<char>], shape: &Shape, (x, y): (usize, usize)) {
    match shape {
        Shape::Horizontal => [0, 1, 2, 3]
            .iter()
            .for_each(|delta| grid[y][x + delta] = '#'),
        Shape::Vertical => [0, 1, 2, 3]
            .iter()
            .for_each(|delta| grid[y - delta][x] = '#'),
        Shape::Square => [0, 1].iter().for_each(|delta| {
            grid[y][x + delta] = '#';
            grid[y - 1][x + delta] = '#';
        }),
        Shape::ReverseL => [0, 1, 2].iter().for_each(|delta| {
            grid[y - 2][x + delta] = '#';
            grid[y - delta][x + 2] = '#';
        }),
        Shape::Plus => {
            grid[y][x + 1] = '#';
            grid[y - 2][x + 1] = '#';
            [0, 1, 2]
                .iter()
                .for_each(|delta| grid[y - 1][x + delta] = '#');
        }
    }
}

fn expand_grid(grid: &mut Vec<Vec<char>>, rows: usize) {
    for _ in grid.len()..rows {
        let mut row = vec!['.'; 9];
        row[0] = '|';
        row[8] = '|';
        grid.push(row);
    }
}

// returns list of offsets from top to first figure on grid
fn height_offsets(grid: &[Vec<char>], top: usize) -> Vec<usize> {
    (1..8)
        .map(|x| {
            let mut delta = 0;
            while grid[top - delta][x] == '.' {
                delta += 1;
            }
            delta
        })
        .collect::<Vec<_>>()
}

fn run_tetris(number_of_figures: usize) -> (usize, HashMap<usize, usize>, (usize, usize)) {
    let input_data = include_str!("../../input/day17").as_bytes();
    let shapes = [
        Shape::Horizontal,
        Shape::Plus,
        Shape::ReverseL,
        Shape::Vertical,
        Shape::Square,
    ];
    let mut grid = vec![vec!['-'; 9]];

    let mut cycle_map: HashMap<(usize, usize, Vec<usize>), Vec<(usize, usize)>> = HashMap::new();
    let mut cycle_set: HashSet<(usize, usize)> = HashSet::new();
    let mut heights: HashMap<usize, usize> = HashMap::new();

    let mut floor = 0;
    let mut op_idx = 0;

    for figure_count in 0..number_of_figures {
        let shape = shapes[figure_count % shapes.len()];
        let new_top = floor + 4 + height(&shape);
        let (mut y, mut x) = (new_top, 3usize);
        expand_grid(&mut grid, new_top);
        while can_move_down(&grid, &shape, (x, y)) {
            y -= 1;
            let op = input_data[op_idx % input_data.len()];
            op_idx += 1;
            if op == b'<' && can_move_left(&grid, &shape, (x, y)) {
                x -= 1;
            } else if op == b'>' && can_move_right(&grid, &shape, (x, y)) {
                x += 1;
            }
        }
        render_on_grid(&mut grid, &shape, (x, y));
        floor = max(y, floor);

        // Save intermediate values for further reference
        // mapping figure_count -> height
        heights.insert(figure_count, floor);

        // Insert touple (op_idx, figure_idx, [diff from top to first figure])
        let key = (
            op_idx % input_data.len(),
            figure_count % shapes.len(),
            height_offsets(&grid, floor),
        );
        // Touple maps to -> vec![(current height, current figure count)]
        match cycle_map.entry(key) {
            Entry::Vacant(e) => {
                e.insert(vec![(floor, figure_count)]);
            }
            Entry::Occupied(mut e) => {
                let (last_floor, last_count) = e.get_mut().pop().unwrap();
                cycle_set.insert((floor - last_floor, figure_count - last_count));
                e.get_mut().push((floor, figure_count));
            }
        };
    }
    assert!(cycle_set.len() == 1);

    (floor, heights, *cycle_set.iter().next().unwrap())
}

fn main() {
    let (height_p1, _, _) = run_tetris(2022);
    println!("Height p1: {height_p1}");

    let (_, heights, (floor_gain, figures_gain)) = run_tetris(4000);
    let cycle_rep = 1000000000000 / figures_gain;
    let cycle_rem = 1000000000000 % figures_gain;

    let part_1 = heights.get(&(cycle_rem + figures_gain - 1)).unwrap();
    let part_2 = (cycle_rep - 1) * floor_gain;
    let height_p2 = part_1 + part_2;
    println!("Height p2: {height_p2}");
}
