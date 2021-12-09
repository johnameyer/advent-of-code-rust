use std::collections::HashMap;

pub enum Instruction {
    SetMask(String),
    SetArr(u64, u64)
}

type GeneratorResult = Vec<Instruction>;

#[aoc_generator(day14)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
    .split("\n")
    .map(|l| {
        let split: Vec<&str> = l.split(" = ").collect();
        if split[0].starts_with("mem") {
            Instruction::SetArr(split[0][4..(split[0].len()-1)].parse().unwrap(), split[1].parse().unwrap())
        } else if split[0].starts_with("mask") {
            Instruction::SetMask(split[1].parse().unwrap())
        } else {
            panic!();
        }
    }).collect()
}

#[aoc(day14, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    let mut values: HashMap<u64, u64> = HashMap::new();
    let mut mask: &String = &"".to_string();
    for instruction in input {
        if let Instruction::SetMask(new_mask) = instruction {
            mask = new_mask;
        } else if let Instruction::SetArr(index, value) = instruction {
            let mut actual_value = *value;
            for (index, char) in mask.chars().enumerate() {
                let actual_index = mask.len() - index - 1;
                actual_value = match char {
                    '1' => actual_value | (1 << actual_index),
                    '0' => actual_value & !(1 << actual_index),
                    _ => actual_value
                }
            }
            values.insert(*index, actual_value);
        }
    }
    values.values().sum()
}

#[aoc(day14, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let mut values: HashMap<u64, u64> = HashMap::new();
    let mut mask: &String = &"".to_string();
    for instruction in input {
        if let Instruction::SetMask(new_mask) = instruction {
            mask = new_mask;
        } else if let Instruction::SetArr(original_index, value) = instruction {
            let mut indices: Vec<u64> = Vec::new();
            indices.push(0);
            for (index, char) in mask.chars().enumerate() {
                let actual_index = mask.len() - index - 1;
                indices = match char {
                    '1' => indices.iter().map(|index| (index << 1) | 1).collect(),
                    '0' => indices.iter().map(|index| (index << 1) | ((*original_index >> actual_index) & 1)).collect(),
                    _ => indices.iter().flat_map(|index| vec![(index << 1) | 1, (index << 1)]).collect()
                };
            }
            for index in indices {
                values.insert(index, *value);
            }
        }
    }
    values.values().sum()
}