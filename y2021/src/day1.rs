#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.trim().parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> usize {
    input.windows(2).filter(|x| x[0] < x[1]).count()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> usize {
    input
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count()
}
