
use std::collections::HashSet;
use std::collections::HashMap;

type GeneratorResult = HashMap<String, HashMap<String, u32>>;

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            let split: Vec<&str> = l.split(" ").collect();
            let key = split[0..=1].join(" ");
            let mut value = HashMap::new();
            let mut i = 4;
            while i < split.len() {
                let count = split[i].parse::<u32>();
                if count.is_err() {
                    // we got a 'no other bags'
                    break;
                }
                i = i + 1;
                let items = split[i..=(i+1)].join(" ");
                i = i + 3;
                value.insert(items, count.unwrap());
            }
            (key, value)
        }).collect()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {
    // we could have parsed the input differently instead
    let sets: HashMap<String, HashSet<String>> = input.into_iter()
        .map(|(key, value)| (key.clone(), value.keys().cloned().collect::<HashSet<String>>())).collect();
    let my_bag = "shiny gold".to_string();
    let mut count = 0;
    let mut containing = HashSet::new();
    containing.insert(my_bag);
    while count != containing.len() {
        count = containing.len();
        let new: HashSet<String> = sets.iter()
            .filter(|(key, _value)| !containing.contains(*key))
            .filter(|(_key, value)| containing.intersection(value).next().is_some())
            .map(|(key, _value)| key.clone()).collect();
        containing.extend(new);
    }
    containing.len() - 1
}

fn bags_inside(key: &String, map: &GeneratorResult) -> u32 {
    map.get(key).unwrap().iter().map(|(key, num)| *num * bags_inside(key, map)).sum::<u32>() + 1
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u32 { 
    let my_bag = "shiny gold".to_string();
    bags_inside(&my_bag, input) - 1
}