fn load_numbers() -> Vec<i64> {
    include_str!("../../input/day20")
        .lines()
        .map(|line| line.parse::<i64>().unwrap())
        .collect::<Vec<_>>()
}

fn run_decryption(array: &Vec<i64>, rounds: usize, multiplier: i64) -> Vec<usize> {
    let mut indices = (0..array.len()).collect::<Vec<_>>();
    for _ in 0..rounds {
        array.iter().enumerate().for_each(|(idx, &element)| {
            let position = indices.iter().position(|&i| i == idx).unwrap();
            indices.remove(position);
            let mut new_pos = ((element * multiplier) + position as i64) % (array.len() as i64 - 1);
            if new_pos < 0 {
                new_pos += array.len() as i64 - 1;
            }
            indices.insert(new_pos as usize, idx);
        });
    }
    indices
}

fn calculate_sum(array: &[i64], indices: &Vec<usize>) -> i64 {
    let zero_idx = indices.iter().position(|&idx| array[idx] == 0).unwrap();
    [1000, 2000, 3000]
        .iter()
        .map(|&dist| {
            let num_idx = (zero_idx + dist) % indices.len();
            array[indices[num_idx]]
        })
        .sum::<i64>()
}

fn main() {
    let array = load_numbers();
    let p1_indices = run_decryption(&array, 1, 1);
    let p1 = calculate_sum(&array, &p1_indices);
    println!("p1: {p1}");

    let p2_indices = run_decryption(&array, 10, 811589153);
    let p2 = calculate_sum(&array, &p2_indices) * 811589153;
    println!("p2: {p2}");
}
