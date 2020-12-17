type GeneratorResult = Vec<Vec<Vec<bool>>>;

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> GeneratorResult {
    vec![input
        .split("\n")
        .map(|l| {
            l.chars()
            .map(|c| c == '#')
            .collect()
        }).collect()]
}

fn access(input: &GeneratorResult, x: usize, y: usize, z: usize) -> bool {
    *input.get(x).and_then(|inner| inner.get(y)).and_then(|innermost| innermost.get(z)).unwrap_or(&false)
}

fn state_at(input: &GeneratorResult, x: isize, y: isize, z: isize) -> bool {
    let mut count = 0;
    for x_offset in 0..=2 {
        for y_offset in 0..=2 {
            for z_offset in 0..=2 {
                if access(input, (x as isize + x_offset as isize - 1) as usize, (y as isize + y_offset as isize - 1) as usize, (z as isize + z_offset as isize - 1) as usize) {
                    count += 1;
                }
            }
        }
    }
    if access(input, x as usize, y as usize, z as usize) {
        count == 3 || count == 4
    } else {
        count == 3
    }
}

#[aoc(day17, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {
    let mut state = input.clone();
    for _ in 0..6 {
        let new_state = vec![vec![vec![false; state[0][0].len() + 2]; state[0].len() + 2]; state.len() + 2];
        state = new_state.iter().enumerate()
            .map(|(x, inner)| inner.iter().enumerate()
                .map(|(y, innermost)| innermost.iter().enumerate()
                    .map(|(z, _)| state_at(&state, x as isize - 1, y as isize - 1, z as isize - 1))
                .collect())
            .collect())
        .collect();
    }
    state.iter().map(|inner| inner.iter().map(|inner_most| inner_most.iter().filter(|i| **i).count() as usize).sum::<usize>()).sum::<usize>()
}


fn access2(input: &Vec<GeneratorResult>, w: usize, x: usize, y: usize, z: usize) -> bool {
    *input.get(w).and_then(|inner| inner.get(x)).and_then(|innerer| innerer.get(y)).and_then(|innermost| innermost.get(z)).unwrap_or(&false)
}

fn state_at2(input: &Vec<GeneratorResult>, w: isize, x: isize, y: isize, z: isize) -> bool {
    let mut count = 0;
    for w_offset in 0..=2 {
        for x_offset in 0..=2 {
            for y_offset in 0..=2 {
                for z_offset in 0..=2 {
                    if access2(input, (w as isize + w_offset as isize - 1) as usize, (x as isize + x_offset as isize - 1) as usize, (y as isize + y_offset as isize - 1) as usize, (z as isize + z_offset as isize - 1) as usize) {
                        count += 1;
                    }
                }
            }
        }
    }
    if access2(input, w as usize, x as usize, y as usize, z as usize) {
        count == 3 || count == 4
    } else {
        count == 3
    }
}

#[aoc(day17, part2)]
pub fn solve_part2(input: &GeneratorResult) -> usize {
    let mut state = vec![input.clone()];
    for _ in 0..6 {
        let new_state = vec![vec![vec![vec![false; state[0][0][0].len() + 2]; state[0][0].len() + 2]; state[0].len() + 2]; state.len() + 2];
        state = new_state.iter().enumerate()
            .map(|(w, inner)| inner.iter().enumerate()
                .map(|(x, innerer)| innerer.iter().enumerate()
                    .map(|(y, innermost)| innermost.iter().enumerate()
                        .map(|(z, _)| state_at2(&state, w as isize - 1, x as isize - 1, y as isize - 1, z as isize - 1))
                    .collect())
                .collect())
            .collect())
        .collect();
    }
    state.iter().map(|inner| inner.iter().map(|innerer| innerer.iter().map(|inner_most| inner_most.iter().filter(|i| **i).count() as usize).sum::<usize>()).sum::<usize>()).sum::<usize>()
}