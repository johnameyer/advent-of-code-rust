type Item = (u32, u32);
type GeneratorResult = Vec<Item>;

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            let row = l[0..7].chars().map(|c| c == 'B').fold(0, |sum, next| 2 * sum + next as u32);
            let col = l[7..10].chars().map(|c| c == 'R').fold(0, |sum, next| 2 * sum + next as u32);
            (row, col)
        }).collect()
}

fn seat_id(row: u32, col: u32) -> u32 {
    row * 8 + col
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u32 {    
    input.into_iter()
        .map(|(row, col)| seat_id(*row, *col))
        .max()
        .unwrap_or(0)
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u32 {
    let mut all_ids = input.iter()
        .map(|(row, col)| seat_id(*row, *col))
        .collect::<Vec<u32>>();
    all_ids.sort();
    all_ids
        .windows(2)
        .find(|arr| arr[0] + 2 == arr[1])
        .map(|arr| arr[0] + 1)
        .unwrap_or(0)
}