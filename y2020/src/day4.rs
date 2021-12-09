use std::collections::HashMap;

// TODO consider
// use regex::Regex;
// use lazy_static::lazy_static;
// lazy_static! {
//     static ref ECL: Regex = Regex::new(r"^$").unwrap();
// }

// TODO consider nicer struct

type Item = HashMap<String, String>;
type GeneratorResult = Vec<Item>;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n\n")
        .map(|l| {
            l.split(&[' ', '\n'] as &[char])
            .filter(|s| s.len() > 0 )
            .map(|item| item.split(':'))
            .map(|mut item| (String::from(item.next().unwrap()), String::from(item.next().unwrap())))
            .collect()
        }).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {    
    let expected_fields = vec!["byr","iyr","eyr","hgt","hcl","ecl","pid"];

    input
        .into_iter()
        .filter( |record| expected_fields.iter().all(|key| record.contains_key(&key.to_string())))
        .count()
}

fn valid_record(input: &Item) -> Option<bool> {
    let byr: u32 = input.get("byr")?.parse().unwrap_or(0);
    if byr < 1920 || byr > 2002 {
        return Some(false);
    }
    let iyr: u32 = input.get("iyr")?.parse().unwrap_or(0);
    if iyr < 2010 || iyr > 2020 {
        return Some(false);
    }
    let eyr: u32 = input.get("eyr")?.parse().unwrap_or(0);
    if eyr < 2020 || eyr > 2030 {
        return Some(false);
    }
    
    let hgt: String = input.get("hgt")?.to_string();
    if !hgt.contains("cm") && !hgt.contains("in") {
        return Some(false);
    }
    let height: u32 = hgt[0..hgt.len() - 2].parse().unwrap_or(0);
    if hgt.contains("cm") {
        if !(150..=193).contains(&height) {
            return Some(false);
        }
    } else if hgt.contains("in") {
        if !(59..=76).contains(&height) {
            return Some(false);
        }
    }

    let hcl: &str = input.get("hcl")?;
    if hcl.chars().next().unwrap() != '#' {
        return Some(false);
    }
    for index in 1..=6 {
        let chara = hcl.chars().nth(index)?;
        if !(chara >= '0' && chara <= '9') && !(chara >= 'a' && chara <= 'f') {
            return Some(false);
        }
    }
    
    let ecl: String = input.get("ecl")?.to_string();
    if !"amb blu brn gry grn hzl oth".split(" ").any(|string| string == ecl) {
        return Some(false);
    }
    
    let pid: String = input.get("pid")?.to_string();
    if pid.len() != 9 {
        return Some(false);
    }
    return Some(pid.parse::<u32>().is_ok());
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &GeneratorResult) -> usize {
    input.into_iter().filter(|record| valid_record(record) == Some(true)).count()
}