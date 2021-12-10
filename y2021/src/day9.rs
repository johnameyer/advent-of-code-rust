use std::collections::HashSet;

#[aoc_generator(day9)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|l| {
            l.trim()
                .chars()
                .map(|x| x.to_string().parse().unwrap())
                .collect()
        })
        .collect()
}

fn get(input: &Vec<Vec<u32>>, coords: (usize, usize)) -> Option<u32> {
    input
        .get(coords.1)
        .map(|inner| inner.get(coords.0))
        .flatten()
        .cloned()
}

fn adjacent(input: &Vec<Vec<u32>>, coords: (usize, usize)) -> Vec<(usize, usize)> {
    [(-1isize as usize, 0), (0, -1isize as usize), (1, 0), (0, 1)]
        .into_iter()
        .map(|i| ((coords.0 + i.0) as usize, (coords.1 + i.1) as usize))
        .filter(|j| get(input, *j).is_some())
        .collect()
}

fn low_points(input: &Vec<Vec<u32>>) -> Vec<(usize, usize)> {
    (0..input.len())
        .flat_map(|y| (0..input[y].len()).map(move |x| (x, y)))
        .filter(|i| {
            adjacent(input, *i)
                .into_iter()
                .all(|j| get(input, j).unwrap() > get(input, *i).unwrap())
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn solve_part1(input: &Vec<Vec<u32>>) -> u32 {
    low_points(input)
        .into_iter()
        .map(|x| get(input, x).unwrap() + 1)
        .sum()
}

#[aoc(day9, part2)]
pub fn solve_part2(input: &Vec<Vec<u32>>) -> usize {
    let low_points = low_points(input);
    let mut sizes = Vec::<usize>::new();
    for start in low_points {
        let mut visited = HashSet::<(usize, usize)>::new();
        let mut frontier: Vec<(usize, usize)> = adjacent(input, start)
            .into_iter()
            .filter(|i| get(input, *i) != Some(9))
            .collect();

        while let Some(next) = frontier.pop() {
            if visited.contains(&next) || get(input, next) == Some(9) {
                continue;
            }
            visited.insert(next);
            for candidate in adjacent(input, next).into_iter() {
                frontier.push(candidate);
            }
        }
        sizes.push(visited.len());
    }

    sizes.sort_unstable();

    sizes.iter().rev().take(3).product()
}
