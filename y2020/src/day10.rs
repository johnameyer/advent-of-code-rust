use std::collections::HashMap;
type GeneratorResult = Vec<u64>;

#[aoc_generator(day10)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut vec: GeneratorResult = input
        .split("\n")
        .map(|l| {
            l.parse().unwrap()
        }).collect();
    vec.insert(0, 0);
    vec.sort();
    vec
}

#[aoc(day10, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    let map = input.windows(2)
        .map(|window| window[1] - window[0])
        .fold(HashMap::new(), |mut map, diff| { map.insert(diff, *(map.get(&diff).unwrap_or(&0)) + 1);  return map;});
    (1 + map.get(&3).unwrap()) * map.get(&1).unwrap()
}

fn solve_recursive(input: &GeneratorResult, consideration: usize, memoization: &mut HashMap<usize, u64>) -> u64 {
    if consideration == input.len() - 1 {
        return 1;
    }
    if memoization.get(&consideration).is_some() {
        return *memoization.get(&consideration).unwrap();
    }
    let mut next = consideration + 1;
    let mut sum = 0;
    while next < input.len() && input[next] - input[consideration] <= 3 {
        sum += solve_recursive(input, next, memoization);
        next += 1;
    }
    memoization.insert(consideration, sum);
    sum
}

#[aoc(day10, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    solve_recursive(&input, 0, &mut HashMap::new())
}