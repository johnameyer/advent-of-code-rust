use std::collections::HashMap;

type GeneratorResult = Vec<u64>;

#[aoc_generator(day15)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split(",")
        .map(|l| {
            l.parse().unwrap()
        }).collect()
}

#[aoc(day15, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    let mut last_spoken: HashMap<u64, u64> = HashMap::new();
    let mut previous_spoken: HashMap<u64, u64> = HashMap::new();
    let mut last = 0;
    for round in 0..2020 {
        last = if round < input.len() {
            input[round]
        } else {
            previous_spoken.get(&last).map(|previously| round as u64 - previously - 1).unwrap_or(0) 
        };
        last_spoken.get(&last).and_then(|last_time| previous_spoken.insert(last, *last_time));
        last_spoken.insert(last, round as u64);
    }
    last
}

#[aoc(day15, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let mut last_spoken: HashMap<u64, u64> = HashMap::new();
    let mut previous_spoken: HashMap<u64, u64> = HashMap::new();
    let mut last = 0;
    for round in 0..30000000 {
        last = if round < input.len() {
            input[round]
        } else {
            previous_spoken.get(&last).map(|previously| round as u64 - previously - 1).unwrap_or(0) 
        };
        last_spoken.get(&last).and_then(|last_time| previous_spoken.insert(last, *last_time));
        last_spoken.insert(last, round as u64);
    }
    last
}