
use std::collections::BTreeSet;

type GeneratorResult = Vec<u64>;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            l.parse().unwrap()
        }).collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    input
        .windows(25 + 1)
        .map(|numbers| (numbers[0..25].iter().map(|&i| i).collect::<BTreeSet<u64>>(), numbers[25]))
        .find(|(previous, consideration)| 
            previous.into_iter().find(|&&item| previous.contains(&(*consideration - item))).is_none()
        )
        .map(|(_previous, consideration)| consideration).unwrap()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let target = solve_part1(input);
    (2..input.len()).find_map(|window|
        input
            .windows(window)
            .find(|numbers| numbers.iter().sum::<u64>() == target)
            .map(|numbers| numbers.iter().min().unwrap() + numbers.iter().max().unwrap())
    ).unwrap()
}