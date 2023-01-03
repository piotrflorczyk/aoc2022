use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Clone, Copy, Debug)]
struct Storm {
    y: i32,
    x: i32,
    dir: char,
}

fn parse_grid() -> Vec<Vec<char>> {
    include_str!("../../input/day24")
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

fn get_storms(grid: &Vec<Vec<char>>) -> Vec<Storm> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut storms = Vec::new();
    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if grid[i][j] != '.' {
                storms.push(Storm {
                    y: i as i32,
                    x: j as i32,
                    dir: grid[i][j],
                })
            }
        }
    }
    storms
}

fn simulate_step(grid: &Vec<Vec<char>>, storms: &Vec<Storm>, step: i32) -> Vec<Vec<char>> {
    let (rows, cols) = (grid.len(), grid[0].len());
    let mut new_grid = vec![vec!['.'; cols]; rows];
    for i in 0..cols {
        new_grid[0][i] = grid[0][i];
        new_grid[rows - 1][i] = grid[rows - 1][i];
    }
    for i in 0..rows {
        new_grid[i][0] = grid[i][0];
        new_grid[i][cols - 1] = grid[i][cols - 1];
    }
    for storm in storms {
        let (x, y) = match storm.dir {
            '<' => (
                (storm.x - step - 1).rem_euclid(cols as i32 - 2) + 1,
                storm.y,
            ),
            '>' => (
                (storm.x + step - 1).rem_euclid(cols as i32 - 2) + 1,
                storm.y,
            ),
            '^' => (
                storm.x,
                (storm.y as i32 - step as i32 - 1).rem_euclid(rows as i32 - 2) + 1,
            ),
            'v' => (
                storm.x,
                (storm.y + step - 1).rem_euclid(rows as i32 - 2) + 1,
            ),
            _ => unreachable!(),
        };
        new_grid[y as usize][x as usize] = storm.dir;
    }
    new_grid
}

fn get_all_grids(grid: &Vec<Vec<char>>, storms: &Vec<Storm>, time: i32) -> Vec<Vec<Vec<char>>> {
    (0..time)
        .map(|t| simulate_step(grid, storms, t))
        .collect::<Vec<_>>()
}

fn simulate(
    grids: &[Vec<Vec<char>>],
    start_x: i32,
    start_y: i32,
    end_y: i32,
    start_time: usize,
) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((start_time, start_y, start_x));
    let mut visited = HashSet::new();
    while let Some((time, y, x)) = queue.pop_front() {
        // first time we get to the bottom is our solution (bfs)
        if y == end_y {
            return time;
        }
        let grid_next = &grids[time + 1];
        for (dy, dx) in [(1, 0), (0, 1), (0, -1i32), (-1i32, 0), (0, 0)] {
            if y + dy >= 0
                && y + dy < grid_next.len() as i32
                && x + dx >= 0
                && x + dx < grid_next[0].len() as i32
                && grid_next[(y + dy) as usize][(x + dx) as usize] == '.'
                && !visited.contains(&(time + 1, y + dy, x + dx))
            {
                visited.insert((time + 1, y + dy, x + dx));
                queue.push_back((time + 1, y + dy, x + dx));
            }
        }
    }
    unreachable!()
}
fn main() {
    let grid = parse_grid();
    let storms = get_storms(&grid);
    let all_grids = get_all_grids(&grid, &storms, 2000);

    let start_end_1 = simulate(&all_grids, 1, 0, grid.len() as i32 - 1, 0);
    println!("Start-end (p1): {start_end_1}");
    let end_start = simulate(
        &all_grids,
        grid[0].len() as i32 - 2,
        grid.len() as i32 - 1,
        0,
        start_end_1,
    );
    let start_end_2 = simulate(&all_grids, 1, 0, grid.len() as i32 - 1, end_start);
    println!("Start-end again (p2): {start_end_2}");
}
