use std::collections::HashSet;

pub struct Line {
    instruction: String,
    argument: i32
}
type GeneratorResult = Vec<Line>;

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            let mut split = l.trim().split(" ");
            Line {
                instruction: split.next().unwrap().to_string(),
                argument: split.next().unwrap().parse().unwrap()
            }
        }).collect()
}

fn simulate(input: &GeneratorResult) -> (i32, bool, HashSet<usize>) {
    let mut accumulator = 0;
    let mut position = 0;
    let mut visited = HashSet::new();
    while !visited.contains(&position) && position < input.len() {
        visited.insert(position);
        match input[position].instruction.as_str() {
            "acc" => {
                accumulator += input[position].argument;
                position += 1;
            }
            "jmp" => {
                position = (position as i32 + input[position].argument) as usize;
            }
            "nop" => {
                position += 1;
            }
            _ => panic!()
        }
    }
    (accumulator, position == input.len(), visited)
}

#[aoc(day8, part1)]
pub fn solve_part1(input: &GeneratorResult) -> i32 {
    simulate(input).0
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &GeneratorResult) -> i32 {
    simulate(input).2.into_iter()
        .filter(|position| input[*position].instruction != "acc")
        .find_map(|position| {
            // TODO nicer way to avoid modifying every time?
            let mut new_input: GeneratorResult = input.into_iter()
                .map(|item| Line { instruction: item.instruction.clone(), argument: item.argument } )
                .collect();
            new_input[position].instruction = match new_input[position].instruction.as_str() {
                "jmp" => "nop".to_string(),
                "nop" => "jmp".to_string(),
                _ => panic!()
            };
            let result = simulate(&new_input);
            if result.1 {
                Some(result.0)
            } else {
                None
            }
        }).unwrap_or(0)
}