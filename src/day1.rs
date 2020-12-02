use std::collections::BTreeSet;
use std::collections::HashSet;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| {
            l.trim().parse().unwrap()
        }).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &Vec<u32>) -> u32 {
    // O(n)
    let target: u32 = 2020;
    let half = target / 2;
    let mut seen = HashSet::<u32>::new();
    let find = input.into_iter().find(|val| {
        let bounded = if **val > half { target - **val } else { **val };
        if seen.contains(&bounded) {
            true
        } else {
            seen.insert(bounded);
            false
        }
    });
    if let Some(found) = find {
        (target - found) * found
    } else {
        0
    }
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &Vec<u32>) -> u32 {
    // O(n^2)
    let target: u32 = 2020;
    let mut seen = BTreeSet::<u32>::new();
    for val in input {
        seen.insert(*val);
    }
    let mut result = None;
    for one in seen.clone() {
        for two in seen.clone() {
            if one == two || one + two > target {
                break;
            }
            let other = target - one - two;
            if (other != one && other != two) && seen.contains(&other) {
                result = Some(other * one * two);
                break;
            }
        }
        if let Some(_) = result {
            break;
        }
    }

    if let Some(value) = result {
        value
    } else {
        0
    }
}