use std::collections::HashMap;

#[derive(Debug)]
pub enum Rule {
    LITERAL(String),
    REF(usize),
    UNION(Box<Rule>, Box<Rule>),
    CONCAT(Box<Rule>, Box<Rule>)
}

impl Rule {
    fn parse(l: &str) -> Box<Rule> {
        Box::new(if let Ok(parsed) = l.parse() {
            Rule::REF(parsed)
        } else if l.starts_with("\"") {
            Rule::LITERAL(l[1..(l.len() - 1)].to_string())
        } else {
            let split: Vec<&str> = l.splitn(2, " | ").collect();
            if split.len() == 2 {
                Rule::UNION(Rule::parse(split[0]), Rule::parse(split[1]))
            } else {
                let space_split: Vec<&str> = l.splitn(2, " ").collect();
                Rule::CONCAT(Rule::parse(space_split[0]), Rule::parse(space_split[1]))
            }
        })
    }

    fn matches(&self, rules: &HashMap<usize, Rule>, string: &String, index: usize) -> Vec<usize> {
        match self {
            Rule::LITERAL(literal) => if string[index..].starts_with(literal) { vec![index + literal.len()] } else { vec![] } ,
            Rule::REF(rule) => rules.get(rule).unwrap().matches(rules, string, index),
            Rule::UNION(left, right) => vec![
                left.matches(rules, string, index),
                right.matches(rules, string, index)
            ].iter().flat_map(|i| i).map(|i| *i).collect(),
            Rule::CONCAT(left, right) => left.matches(rules, string, index).iter().flat_map(|new_index| right.matches(rules, string, *new_index)).collect()
        }
    }
}

type GeneratorResult = (HashMap<usize, Rule>, Vec<String>);

#[aoc_generator(day19, part1)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let mut split = input
        .split("\n\n");
    let rules: HashMap<usize, Rule> = split.next().unwrap()
        .split("\n")
        .map(|l| {
            let mut split = l.split(": ");
            (
            split.next().unwrap().parse().unwrap(),
            *Rule::parse(split.next().unwrap())
            )
        }).collect();

    let strings: Vec<String> = split.next().unwrap().split("\n").map(String::from).collect();

    (rules, strings)
}

#[aoc(day19, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {
    let (rules, strings) = input;
    let first_rule = &rules.get(&0).unwrap();
    strings
    .iter()
    .filter(|key| first_rule.matches(rules, key, 0).contains(&(key.len())))
    .count()
}

#[aoc_generator(day19, part2)]
pub fn input_generator2(input: &str) -> GeneratorResult {
    let (mut rules, strings) = input_generator(input);
    rules.insert(8, *Rule::parse("42 | 42 8"));
    rules.insert(11, *Rule::parse("42 31 | 42 11 31"));
    (rules, strings)
}

#[aoc(day19, part2)]
pub fn solve_part2(input: &GeneratorResult) -> usize {
    let (rules, strings) = input;
    let first_rule = &rules.get(&0).unwrap();
    strings
    .iter()
    .filter(|key| first_rule.matches(rules, key, 0).contains(&(key.len())))
    .count()
}
