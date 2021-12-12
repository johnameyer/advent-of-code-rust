use std::collections::{HashMap, HashSet};

#[aoc_generator(day8)]
pub fn input_generator(input: &str) -> Vec<Vec<Vec<HashSet<usize>>>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .split('|')
                .map(|section| {
                    section
                        .trim()
                        .split(" ")
                        .map(|item| item.chars().map(|c| (c as usize - 'a' as usize)).collect())
                        .collect()
                })
                .collect()
        })
        .collect()
}

const SEGMENTS_ILLUMINATED: [usize; 10] = [6, 2, 5, 5, 4, 5, 6, 3, 7, 6];
const UNIQUE: [usize; 4] = [1, 4, 7, 8];
// 0:      1:      2:      3:      4:
// 0000    ....    0000    0000    ....
// 1    2  .    2  .    2  .    2  1    2
// 1    2  .    2  .    2  .    2  1    2
// ....    ....    3333    3333    3333
// 4    5  .    5  4    .  .    5  .    5
// 4    5  .    5  4    .  .    5  .    5
// 6666    ....    6666    6666    ....

//  5:      6:      7:      8:      9:
// 0000    0000    0000    0000    0000
// 1    .  1    .  .    2  1    2  1    2
// 1    .  1    .  .    2  1    2  1    2
// 3333    3333    ....    3333    3333
// .    5  4    5  .    5  4    5  .    5
// .    5  4    5  .    5  4    5  .    5
// 6666    6666    ....    6666    6666
const MAPPINGS: [&[usize]; 10] = [
    &[0, 1, 2, 4, 5, 6],
    &[2, 5],
    &[0, 2, 3, 4, 6],
    &[0, 2, 3, 5, 6],
    &[1, 2, 3, 5],
    &[0, 1, 3, 5, 6],
    &[0, 1, 3, 4, 5, 6],
    &[0, 2, 5],
    &[0, 1, 2, 3, 4, 5, 6],
    &[0, 1, 2, 3, 5, 6],
];

#[aoc(day8, part1)]
pub fn solve_part1(input: &Vec<Vec<Vec<HashSet<usize>>>>) -> usize {
    let values: HashSet<usize> = UNIQUE.iter().map(|i| SEGMENTS_ILLUMINATED[*i]).collect();
    input
        .iter()
        .map(|it| {
            it[1]
                .iter()
                .filter(|item| values.contains(&(item.len())))
                .count()
        })
        .sum()
}

fn get_mappings(input: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    let mut current_mapping: Vec<usize> = vec![8; 7];

    let set = |normalized_index: usize, real_index: usize, mapping: &mut Vec<usize>| {
        mapping[real_index] = normalized_index;
    };

    // first determine top segment (0) by subtracting 1 from 7
    set(
        0,
        *input
            .iter()
            .find(|it| it.len() == 3)
            .unwrap()
            .iter()
            .find(|i| !input.iter().find(|it| it.len() == 2).unwrap().contains(i))
            .unwrap(),
        &mut current_mapping,
    );

    // now proceed by occurrence uniqueness - segments 1 4 and 5 appear a unique number of times
    let source_counts: Vec<usize> = (0..7)
        .map(|i| MAPPINGS.iter().filter(|it| it.contains(&i)).count())
        .collect();
    let counts: Vec<usize> = (0..7)
        .map(|i| input.iter().filter(|it| it.contains(&i)).count())
        .collect();

    // segment 2 appears the same number of times as segment 0 but we have determined that one already
    for i in [1, 4, 5] {
        set(
            i,
            counts
                .iter()
                .enumerate()
                .find(|it| *it.1 == source_counts[i])
                .unwrap()
                .0,
            &mut current_mapping,
        );
    }

    // segment 2 appears the same number of times as segment 0 but we have determined that one already
    set(
        2,
        counts
            .iter()
            .enumerate()
            .find(|it| *it.1 == source_counts[2] && current_mapping[it.0] != 0)
            .unwrap()
            .0,
        &mut current_mapping,
    );

    // segment 3 appears in 4 but we have already determined the other members
    set(
        3,
        *input
            .iter()
            .find(|it| it.len() == source_counts[4])
            .unwrap()
            .iter()
            .find(|it| current_mapping[**it] == 8)
            .unwrap(),
        &mut current_mapping,
    );

    // segment 6 is remaining
    set(
        6,
        current_mapping
            .iter()
            .enumerate()
            .find(|it| *it.1 == 8)
            .unwrap()
            .0,
        &mut current_mapping,
    );

    let result: Vec<HashSet<usize>> = MAPPINGS
        .iter()
        .map(|item| {
            item.iter()
                .map(|entry| current_mapping.iter().position(|it| *it == *entry).unwrap())
                .collect::<HashSet<usize>>()
        })
        .collect();

    result
}

#[aoc(day8, part2)]
pub fn solve_part2(input: &Vec<Vec<Vec<HashSet<usize>>>>) -> u32 {
    input
        .iter()
        .map(|row| {
            let mappings = get_mappings(&row[0]);
            row[1]
                .iter()
                .map(|digit| mappings.iter().position(|it| it == digit).unwrap())
                .fold(0, |acc, i| 10 * acc + i as u32)
        })
        .sum()
}
