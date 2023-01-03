fn process(matrix: &[Vec<u8>], visited: &mut [Vec<bool>]) {
    for outer in 0..matrix.len() {
        let mut tallest = 0;
        visited[outer][0] = true;
        for inner in 0..matrix.len() {
            if matrix[outer][inner] > tallest {
                tallest = matrix[outer][inner];
                visited[outer][inner] = true;
            }
        }
    }
    for outer in 0..matrix.len() {
        let mut tallest = 0;
        visited[outer][matrix.len() - 1] = true;
        for inner in (0..matrix.len()).rev() {
            if matrix[outer][inner] > tallest {
                tallest = matrix[outer][inner];
                visited[outer][inner] = true;
            }
        }
    }
    for outer in 0..matrix.len() {
        let mut tallest = 0;
        visited[0][outer] = true;
        for inner in 0..matrix.len() {
            if matrix[inner][outer] > tallest {
                tallest = matrix[inner][outer];
                visited[inner][outer] = true;
            }
        }
    }
    for outer in 0..matrix.len() {
        let mut tallest = 0;
        visited[matrix.len() - 1][outer] = true;
        for inner in (0..matrix.len()).rev() {
            if matrix[inner][outer] > tallest {
                tallest = matrix[inner][outer];
                visited[inner][outer] = true;
            }
        }
    }
}

fn load_grid() -> Vec<Vec<u8>> {
    let lines = include_str!("../../input/day8").lines();
    lines
        .map(|line| {
            line.chars()
                .map(|char| char.to_string().parse::<u8>().unwrap())
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>()
}

fn find_visible() -> usize {
    let matrix = load_grid();
    let mut visited = vec![vec![false; matrix.len()]; matrix.len()];

    process(&matrix, &mut visited);

    visited
        .iter()
        .map(|v| v.iter().filter(|&b| *b).count())
        .sum::<usize>()
}

fn in_bound(x: i32, y: i32, size: i32) -> bool {
    x >= 0 && y >= 0 && x < size && y < size
}

fn calculate_tree_score(matrix: &Vec<Vec<u8>>, startx: usize, starty: usize) -> usize {
    let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];
    let tree_height = matrix[startx][starty];
    return directions.iter().fold(1, |acc, dir| {
        let (x_iter, y_iter) = dir;
        let (mut x, mut y) = (startx as i32, starty as i32);
        let mut trees_count = 0;
        x += x_iter;
        y += y_iter;
        while in_bound(x, y, matrix.len() as i32) && matrix[x as usize][y as usize] < tree_height {
            trees_count += 1;
            x += x_iter;
            y += y_iter;
        }
        if in_bound(x, y, matrix.len() as i32) {
            trees_count += 1;
        }
        acc * trees_count
    });
}

fn find_best_tree() -> usize {
    let matrix = load_grid();
    matrix
        .iter()
        .enumerate()
        .map(|(row_idx, row)| {
            row.iter()
                .enumerate()
                .map(|(col_idx, _)| calculate_tree_score(&matrix, row_idx, col_idx))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let p1_visible = find_visible();
    println!("p1: {p1_visible}");
    let p2_best_tree = find_best_tree();
    println!("p2: {p2_best_tree}");
}
