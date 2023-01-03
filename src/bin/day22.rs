#[derive(Copy, Clone, Debug)]
enum Instr {
    Rotate(char),
    Rep(u8),
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn dirx(&self) -> i32 {
        match *self {
            Direction::Right => 1,
            Direction::Left => -1,
            _ => 0,
        }
    }
    fn diry(&self) -> i32 {
        match *self {
            Direction::Down => 1,
            Direction::Up => -1,
            _ => 0,
        }
    }
    fn rotate_right(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
    fn rotate_left(&self) -> Direction {
        match *self {
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Up => Direction::Left,
        }
    }
    fn score(&self) -> i32 {
        match *self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

fn tokenize_instructions(instr: &str) -> Vec<Instr> {
    let mut token_idx = 0;
    let mut current = 0;
    let mut instructions = Vec::new();
    let instr_stream = instr.chars().collect::<Vec<_>>();
    while token_idx < instr_stream.len() {
        if instr_stream[token_idx].is_numeric() {
            current = current * 10 + instr_stream[token_idx].to_digit(10).unwrap();
        } else {
            instructions.push(Instr::Rep(current as u8));
            instructions.push(Instr::Rotate(instr_stream[token_idx]));
            current = 0;
        }
        token_idx += 1;
    }
    if current != 0 {
        instructions.push(Instr::Rep(current as u8));
    }
    instructions
}

fn parse() -> (Vec<Vec<char>>, Vec<Instr>) {
    let input = include_str!("../../input/day22");
    let max_width = input.lines().max_by_key(|line| line.len()).unwrap().len() + 2;
    let max_height = input.lines().count() - 2;
    let lines = input.lines().collect::<Vec<_>>();

    let mut grid = Vec::new();
    grid.push(vec![' '; max_width]);
    lines[0..max_height].iter().for_each(|ln| {
        let mut chars = vec![' '; 1];
        chars.extend(ln.chars());
        chars.extend(vec![' '; max_width - ln.len() - 1]);
        grid.push(chars);
    });
    grid.push(vec![' '; max_width]);
    let instructions = tokenize_instructions(lines[lines.len() - 1]);
    (grid, instructions)
}

fn get_start_pos(grid: &[Vec<char>]) -> (i32, i32) {
    (
        1,
        grid[1]
            .iter()
            .position(|&ch| ch != ' ' && ch != '#')
            .unwrap() as i32,
    )
}

fn wrap_simple(grid: &[Vec<char>], (y, x): (i32, i32), dir: Direction) -> (i32, i32, Direction) {
    let (mut seekx, mut seeky) = match dir {
        Direction::Right => (0, y),
        Direction::Left => (grid[0].len() as i32 - 1, y),
        Direction::Down => (x, 0),
        Direction::Up => (x, grid.len() as i32 - 1),
    };
    while grid[seeky as usize][seekx as usize] == ' ' {
        seeky += dir.diry();
        seekx += dir.dirx();
    }
    (seeky, seekx, dir)
}

fn wrap(_grid: &[Vec<char>], (y, x): (i32, i32), dir: Direction) -> (i32, i32, Direction) {
    let cube_size = 50;
    let (cube_y, cube_x) = ((y - 1) % cube_size, (x - 1) % cube_size);
    let (new_x, new_y, new_dir) = match ((y - 1) / cube_size, (x - 1) / cube_size, dir) {
        (0, 1, Direction::Left) => (3 * cube_size - 1 - cube_y, 0, Direction::Right),
        (0, 1, Direction::Up) => (3 * cube_size + cube_x, 0, Direction::Right),
        (0, 2, Direction::Right) => (
            3 * cube_size - 1 - cube_y,
            2 * cube_size - 1,
            Direction::Left,
        ),
        (0, 2, Direction::Down) => (cube_size + cube_x, 2 * cube_size - 1, Direction::Left),
        (0, 2, Direction::Up) => (4 * cube_size - 1, cube_x, Direction::Up),
        (1, 1, Direction::Left) => (2 * cube_size, cube_y, Direction::Down),
        (1, 1, Direction::Right) => (cube_size - 1, 2 * cube_size + cube_y, Direction::Up),
        (2, 0, Direction::Left) => (cube_size - 1 - cube_y, cube_size, Direction::Right),
        (2, 0, Direction::Up) => (cube_size + cube_x, cube_size, Direction::Right),
        (2, 1, Direction::Right) => (cube_size - 1 - cube_y, 3 * cube_size - 1, Direction::Left),
        (2, 1, Direction::Down) => (3 * cube_size + cube_x, cube_size - 1, Direction::Left),
        (3, 0, Direction::Left) => (0, cube_size + cube_y, Direction::Down),
        (3, 0, Direction::Right) => (3 * cube_size - 1, cube_size + cube_y, Direction::Up),
        (3, 0, Direction::Down) => (0, 2 * cube_size + cube_x, Direction::Down),
        _ => unreachable!(),
    };
    (new_x + 1, new_y + 1, new_dir)
}

fn move_until_limit(
    grid: &[Vec<char>],
    (starty, startx): (i32, i32),
    start_dir: Direction,
    reps: u8,
    p2: bool,
) -> (i32, i32, Direction) {
    let (mut x, mut y) = (startx, starty);
    let mut dir = start_dir;
    for _ in 0..reps {
        let next_cell = grid[(y + dir.diry()) as usize][(x + dir.dirx()) as usize];
        if next_cell == '#' {
            break;
        } else if next_cell == ' ' {
            let (new_y, new_x, new_dir) = if p2 {
                wrap(grid, (y, x), dir)
            } else {
                wrap_simple(grid, (y, x), dir)
            };
            if grid[new_y as usize][new_x as usize] == '#' {
                break;
            }
            (y, x, dir) = (new_y, new_x, new_dir);
        } else {
            y += dir.diry();
            x += dir.dirx();
        }
    }
    (y, x, dir)
}

fn run_instructions(
    grid: &[Vec<char>],
    instructions: &Vec<Instr>,
    p2: bool,
) -> (i32, i32, Direction) {
    let (mut y, mut x) = get_start_pos(grid);
    let mut direction = Direction::Right;
    for instr in instructions {
        match instr {
            Instr::Rotate(ch) => {
                if *ch == 'R' {
                    direction = direction.rotate_right();
                } else if *ch == 'L' {
                    direction = direction.rotate_left();
                }
            }
            Instr::Rep(n) => {
                (y, x, direction) = move_until_limit(grid, (y, x), direction, *n, p2);
            }
        };
    }
    (y, x, direction)
}

fn main() {
    let (grid, instructions) = parse();
    let (y, x, direction) = run_instructions(&grid, &instructions, false);
    let p1_score = 1000 * y + 4 * x + direction.score();
    println!("p1: {p1_score}");
    let (p2_y, p2_x, p2_direction) = run_instructions(&grid, &instructions, true);
    let p2_score = 1000 * p2_y + 4 * p2_x + p2_direction.score();
    println!("p2: {p2_score}");
}
