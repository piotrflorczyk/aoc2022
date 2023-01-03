use std::collections::BinaryHeap;

fn parse_input(start_symbol: u8) -> ((usize, usize), Vec<(usize, usize)>, Vec<Vec<u8>>) {
    let mut start = vec![];
    let mut end = (0, 0);
    let grid = include_str!("../../input/day12")
        .lines()
        .enumerate()
        .map(|(row, line)| {
            line.bytes()
                .enumerate()
                .map(|(col, ch)| {
                    if ch == b'S' || ch == start_symbol {
                        start.push((row, col));
                        b'a'
                    } else if ch == b'E' {
                        end = (row, col);
                        b'z'
                    } else {
                        ch
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    (end, start, grid)
}

fn get_shortest_distance(start_symbol: u8) -> u32 {
    let (end, start, grid) = parse_input(start_symbol);
    let mut distances = vec![vec![u32::MAX; grid[0].len()]; grid.len()];

    let mut pq = BinaryHeap::new();
    // (path_distance, coordinates:(x, y))
    start.iter().for_each(|(x, y)| {
        distances[*x][*y] = 0;
        pq.push((0i32, (*x, *y)));
    });

    while !pq.is_empty() {
        let (distance, (x, y)) = pq.pop().unwrap();
        for dir in [(1i32, 0i32), (-1, 0), (0, 1), (0, -1)] {
            let new_x = x as i32 + dir.0;
            let new_y = y as i32 + dir.1;
            if new_x >= 0
                && new_x < (grid.len() as i32)
                && new_y >= 0
                && new_y < (grid[0].len() as i32)
                && distances[new_x as usize][new_y as usize] > distances[x][y] + 1
                && grid[new_x as usize][new_y as usize] as i16 - grid[x][y] as i16 <= 1
            {
                distances[new_x as usize][new_y as usize] = distances[x][y] + 1;
                pq.push((distance - 1, (new_x as usize, new_y as usize)));
            }
        }
    }
    distances[end.0][end.1]
}

fn main() {
    let p1 = get_shortest_distance(b'S');
    println!("p1: {p1}");
    let p2 = get_shortest_distance(b'a');
    println!("p2: {p2}");
}
