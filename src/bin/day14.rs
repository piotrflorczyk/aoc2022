#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn normalize_dir(dir: i32) -> i32 {
    match dir.cmp(&0) {
        std::cmp::Ordering::Greater => 1,
        std::cmp::Ordering::Less => -1,
        std::cmp::Ordering::Equal => 0,
    }
}

fn move_sand(grid: &[Vec<char>], sand: &Point, min_x: usize) -> Option<Point> {
    // can fall straight down
    if sand.y + 1 < grid.len() && grid[sand.y + 1][sand.x - min_x] == '.' {
        Some(Point {
            x: sand.x,
            y: sand.y + 1,
        })
    } else if sand.y + 1 < grid.len()
        && sand.x as i32 - min_x as i32 - 1 >= 0
        && grid[sand.y + 1][sand.x - min_x - 1] == '.'
    {
        Some(Point {
            x: sand.x - 1,
            y: sand.y + 1,
        })
    } else if sand.y + 1 < grid.len()
        && sand.x - min_x + 1 < grid[0].len()
        && grid[sand.y + 1][sand.x - min_x + 1] == '.'
    {
        Some(Point {
            x: sand.x + 1,
            y: sand.y + 1,
        })
    } else {
        None
    }
}

fn fallen_of_the_grid(grid: &[Vec<char>], sand: &Point, min_x: usize, max_x: usize) -> bool {
    sand.y + 2 >= grid.len() || sand.x == min_x || sand.x >= max_x
}

fn parse_points() -> Vec<Vec<Point>> {
    let lines = include_str!("../../input/day14").lines();
    lines
        .map(|line| {
            line.split(" -> ")
                .map(|point| {
                    let (x, y) = point.split_once(',').unwrap();
                    Point {
                        x: x.parse::<usize>().unwrap(),
                        y: y.parse::<usize>().unwrap(),
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn get_limits(points: &[Vec<Point>]) -> (usize, usize, usize) {
    (
        points
            .iter()
            .flatten()
            .min_by_key(|point| point.x)
            .unwrap()
            .x as usize,
        points
            .iter()
            .flatten()
            .max_by_key(|point| point.x)
            .unwrap()
            .x as usize,
        points
            .iter()
            .flatten()
            .max_by_key(|point| point.y)
            .unwrap()
            .y as usize,
    )
}

fn simulate_sand(p2: bool) -> usize {
    let moves = parse_points();
    let (min_x, max_x, max_y) = get_limits(&moves);
    let mut grid = vec![vec!['.'; 2 * max_x]; max_y + 3];

    // add floor
    for i in 0..grid[0].len() {
        grid[max_y + 2][i] = '#';
    }
    moves.iter().for_each(|line| {
        let mut prev = 0;
        for i in 1..line.len() {
            let dir_x = normalize_dir(line[i].x as i32 - line[prev].x as i32);
            let dir_y = normalize_dir(line[i].y as i32 - line[prev].y as i32);
            let mut x = line[prev].x;
            let mut y = line[prev].y;
            while x != line[i].x || y != line[i].y {
                grid[y][x] = '#';
                x = (x as i32 + dir_x) as usize;
                y = (y as i32 + dir_y) as usize;
            }
            grid[y][x] = '#';
            prev = i;
        }
    });

    let mut iter = 0;
    loop {
        let mut sand = Point { x: 500, y: 0 };
        while let Some(new_point) = move_sand(&grid, &sand, 0) {
            sand = new_point;
        }
        if !p2 && fallen_of_the_grid(&grid, &sand, min_x, max_x) {
            return iter;
        }
        grid[sand.y][sand.x] = 'o';
        iter += 1;
        if p2 && sand.x == 500 && sand.y == 0 {
            return iter;
        }
    }
}

fn main() {
    let iter_p1 = simulate_sand(false);
    println!("p1: {iter_p1}");
    let iter_p2 = simulate_sand(true);
    println!("p2: {iter_p2}");
}
