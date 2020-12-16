
use core::ops::RangeInclusive;
use std::collections::HashMap;

type GeneratorResult = (HashMap<String, Vec<RangeInclusive<u32>>>, Vec<u32>, Vec<Vec<u32>>);

#[aoc_generator(day16)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let split = input
        .split("\n\n")
        .collect::<Vec<&str>>();
    let keys = split[0]
        .lines()
        .map(|l| {
            let mut split = l.split(": ");
            (split.next().unwrap().to_string(), split.next().unwrap().split(" or ").map(|range| range.split("-")).map(|mut range| (range.next().unwrap().parse().unwrap()..=range.next().unwrap().parse().unwrap())).collect())
        }).collect();
    
    let your = split[1].split("\n").nth(1).unwrap().split(",").map(|num| num.parse().unwrap()).collect();
        
    let nearby = split[2].split("\n").skip(1).map(|options| options.split(",").map(|option| option.parse().unwrap()).collect()).collect();

    (keys, your, nearby)
}

#[aoc(day16, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u32 {
    let all_ranges: Vec<RangeInclusive<u32>> = input.0.iter()
        .flat_map(|tuple| tuple.1.clone())
        .collect();
    input.2.iter()
        .map(|ticket| ticket.iter()
            .filter(|value| !all_ranges.iter()
                .any(|range| range.contains(value)))
            .sum::<u32>())
        .sum()
}

#[aoc(day16, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let all_ranges: Vec<RangeInclusive<u32>> = input.0.iter()
        .flat_map(|tuple| tuple.1.clone())
        .collect();
    let valid = input.2.iter()
        .filter(|ticket| ticket.iter()
            .all(|value| all_ranges.iter().any(|range| range.contains(value))));
    let mut possible_keys: Vec<HashMap<String, Vec<RangeInclusive<u32>>>> = (0..input.1.len())
        .map(|_| input.0.clone())
        .collect();
    for valid in valid {
        for (index, field) in valid.iter().enumerate() {
            possible_keys[index].retain(|_key, ranges| ranges.iter().any(|range| range.contains(field)));
        }
    }

    for _ in 0..possible_keys.len() {
        for index in 0..possible_keys.len() {
            if possible_keys[index].len() == 1 {
                for other in 0..possible_keys.len() {
                    let last = possible_keys[index].keys().next().unwrap().clone();
                    if other != index {
                        possible_keys[other].remove(&last);
                    }
                }
            }
        }
        if possible_keys.iter().all(|possible| possible.len() == 1) {
            break;
        }
    }

    // println!("{:?}", possible_keys.iter().map(|vec| vec.iter().map(|tuple| tuple.0.clone()).collect()).collect::<Vec<Vec<String>>>());

    input.1.iter()
        .enumerate()
        .filter(|(index, _field)| possible_keys[*index].keys().any(|key| key.starts_with("departure")))
        .map(|(_index, field)| *field as u64)
        .product::<u64>()
}