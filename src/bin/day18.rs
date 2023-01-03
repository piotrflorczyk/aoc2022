#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    z: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cube {
    Unknown,
    Air,
    Lava,
}

fn crete_grid() -> (Vec<Vec<Vec<Cube>>>, Vec<Point>) {
    let points = include_str!("../../input/day18")
        .lines()
        .map(|line| {
            let numbers = line.split(',').collect::<Vec<_>>();
            Point {
                x: numbers[0].parse::<usize>().unwrap(),
                y: numbers[1].parse::<usize>().unwrap(),
                z: numbers[2].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<_>>();

    let max_x = points.iter().max_by_key(|point| point.x).unwrap().x;
    let max_y = points.iter().max_by_key(|point| point.y).unwrap().y;
    let max_z = points.iter().max_by_key(|point| point.z).unwrap().z;

    let mut space = vec![vec![vec![Cube::Unknown; max_x + 3]; max_y + 3]; max_z + 3];
    points.iter().for_each(|point| {
        space[point.z + 1][point.y + 1][point.x + 1] = Cube::Lava;
    });
    (space, points)
}

fn count_surface(
    space: &[Vec<Vec<Cube>>],
    points: &[Point],
    dirs: &[(i32, i32, i32)],
    cube_type: Cube,
) -> u32 {
    points.iter().fold(0, |acc, point| {
        acc + dirs
            .iter()
            .filter(|(dir_x, dir_y, dir_z)| {
                space[(point.z as i32 + dir_z + 1) as usize][(point.y as i32 + dir_y + 1) as usize]
                    [(point.x as i32 + dir_x + 1) as usize]
                    == cube_type
            })
            .count() as u32
    })
}

fn main() {
    let (mut space, points) = crete_grid();
    let dirs = [
        (1, 0, 0),
        (-1, 0, 0),
        (0, 1, 0),
        (0, -1, 0),
        (0, 0, 1),
        (0, 0, -1),
    ];
    let count_p1 = count_surface(&space, &points, &dirs, Cube::Unknown);
    println!("p1: {count_p1}");

    // mark reachable air
    let mut queue = vec![(0i32, 0i32, 0i32)];
    while !queue.is_empty() {
        let (z, y, x) = queue.pop().unwrap();
        for (dirz, diry, dirx) in dirs {
            if z + dirz >= 0
                && y + diry >= 0
                && x + dirx >= 0
                && z + dirz < space.len() as i32
                && y + diry < space[0].len() as i32
                && x + dirx < space[0][0].len() as i32
                && space[(z + dirz) as usize][(y + diry) as usize][(x + dirx) as usize]
                    == Cube::Unknown
            {
                space[(z + dirz) as usize][(y + diry) as usize][(x + dirx) as usize] = Cube::Air;
                queue.push((z + dirz, y + diry, x + dirx));
            }
        }
    }
    let count_p2 = count_surface(&space, &points, &dirs, Cube::Air);

    println!("p2: {count_p2}");
}
