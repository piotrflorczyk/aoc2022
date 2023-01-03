fn get_overlaping(p2: bool) -> usize {
    include_str!("../../input/day4")
        .lines()
        .filter(|line| {
            let (left, right) = line.split_once(',').unwrap();
            let (left1_str, right1_str) = left.split_once('-').unwrap();
            let (left2_str, right2_str) = right.split_once('-').unwrap();
            let (left1, right1) = (
                left1_str.parse::<i16>().unwrap(),
                right1_str.parse::<i16>().unwrap(),
            );
            let (left2, right2) = (
                left2_str.parse::<i16>().unwrap(),
                right2_str.parse::<i16>().unwrap(),
            );
            if p2 {
                left2 <= right1 && right2 >= left1 || left1 <= right2 && right1 >= left2
            } else {
                left1 >= left2 && right1 <= right2 || left2 >= left1 && right2 <= right1
            }
        })
        .count()
}

fn main() {
    let p1 = get_overlaping(false);
    println!("p1: {p1}");
    let p2 = get_overlaping(true);
    println!("p2: {p2}");
}
