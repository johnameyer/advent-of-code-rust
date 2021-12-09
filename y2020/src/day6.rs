
use std::collections::HashMap;
use std::collections::BTreeSet;

type Item = BTreeSet<char>;
type GeneratorResult = Vec<Item>;

#[aoc_generator(day6, part1)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n\n")
        .map(|l| {
            l.chars()
                .filter(|c| *c != '\n')
                .collect()
        }).collect()
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {    
    input.into_iter()
        .map(|set| set.iter().count())
        .sum()
}

#[aoc_generator(day6, part2)]
pub fn input_generator2(input: &str) -> GeneratorResult {
    input
        .split("\n\n")
        .map(|l| {
            let mut map: HashMap<char, usize> = HashMap::new();
            for ch in l.chars() {
                map.insert(ch, *map.get(&ch).unwrap_or(&(0 as usize)) + 1);
            }
            let expect = l.chars().filter(|c| *c == '\n').count() + 1;
            map.into_iter()
                .filter(|(key, value)| *key != '\n' && *value == expect)
                .map(|(key, _value)| key)
                .collect()
        }).collect()
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &GeneratorResult) -> usize {    
    input.into_iter()
        .map(|set| set.iter().count())
        .sum()
}