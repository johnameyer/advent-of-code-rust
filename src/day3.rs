#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|l| {
            l.trim().chars().into_iter().map(|c| c == '#' ).collect()
        }).collect::<Vec<Vec<bool>>>()
}

fn trees_along_path(input: &Vec<Vec<bool>>, right: usize, down: usize) -> u32 {
    let mut x = right;
    let mut y = down;
    let mut count = 0;
    let true_width = input[y].len();
    while y < input.len() {
        if input[y][x % true_width] {
            count = count + 1
        }
        y = y + down;
        x = x + right;
    }
    count
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &Vec<Vec<bool>>) -> u32 {
    trees_along_path(input, 3, 1)
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &Vec<Vec<bool>>) -> u32 {
    vec![(1,1), (3,1), (5,1), (7,1), (1,2)].into_iter().map(|(right, down)| trees_along_path(input, right, down)).product()
}