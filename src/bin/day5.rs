fn parse_stacks() -> Vec<Vec<char>> {
    let lines = include_str!("../../input/day5").lines().collect::<Vec<_>>();
    let no_stacks = (lines[0].len() + 1) / 4;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); no_stacks];

    lines
        .iter()
        .take_while(|line| !line.starts_with(" 1"))
        .for_each(|line| {
            let chunks = line.as_bytes().chunks(4);
            chunks.enumerate().for_each(|(idx, chunk)| {
                let chunk_s = std::str::from_utf8(chunk).unwrap();
                if !chunk_s.starts_with("   ") {
                    stacks[idx].push(chunk[1] as char);
                }
            });
        });
    stacks.iter_mut().for_each(|s| {
        s.reverse();
    });
    stacks
}

fn get_top_values(stacks: &[Vec<char>]) -> String {
    let top_values = stacks
        .iter()
        .map(|stack| *stack.last().unwrap())
        .collect::<Vec<_>>();

    top_values.iter().collect::<String>()
}

fn rearange(p2: bool) -> Vec<Vec<char>> {
    let mut stacks = parse_stacks();
    include_str!("../../input/day5")
        .lines()
        .skip_while(|line| !line.starts_with("move"))
        .for_each(|line| {
            let splits = line
                .split(' ')
                .flat_map(|c| c.parse::<usize>())
                .collect::<Vec<_>>();
            let (number, from, to) = (splits[0], splits[1] - 1, splits[2] - 1);
            if p2 {
                let start = stacks[from].len() - number;
                let mut range = stacks[from][start..].to_vec();
                stacks[to].append(&mut range);
                stacks[from].truncate(start);
            } else {
                for _ in 0..number {
                    let ch = stacks[from].pop().unwrap();
                    stacks[to].push(ch);
                }
            }
        });
    stacks
}

fn main() {
    let p1_stacks = rearange(false);
    let p1 = get_top_values(&p1_stacks);
    println!("p1: {p1}");
    let p2_stacks = rearange(true);
    let p2 = get_top_values(&p2_stacks);
    println!("p2: {p2}");
}
