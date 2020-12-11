type GeneratorResult = Vec<Vec<char>>;

#[aoc_generator(day11)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            l.chars().collect()
        }).collect()
}

fn access(input: &GeneratorResult, row: usize, col: usize, row_offset: isize, col_offset: isize) -> char {
    let actual_row = (row as isize + row_offset) as usize;
    let actual_col = (col as isize + col_offset) as usize;
    if (0..input.len()).contains(&actual_row) && (0..input[0].len()).contains(&actual_col) {
        input[actual_row][actual_col]
    } else {
        '!'
    }
}

fn simulate(input: &GeneratorResult) -> (GeneratorResult, bool) {
    let mut changed = false;
    let result: GeneratorResult = (0..input.len()).map(|row| (0..input[row].len()).map(|col| {
        let to_be = match input[row][col] {
            'L' => {
                if (-1..=1).any(|row_offset| (-1..=1).any(|col_offset| access(input, row, col, row_offset, col_offset) == '#')) {'L'} else {'#'}
            }
            '#' => {
                if (-1..=1).map(|row_offset| (-1..=1).filter(|col_offset| access(input, row, col, row_offset, *col_offset) == '#').count()).sum::<usize>() < 5 {'#'} else {'L'}
            }
            _ => input[row][col]
        };
        if to_be != input[row][col] {
            changed = true;
        }
        to_be
    }).collect()).collect();
    (result, changed)
}

#[aoc(day11, part1)]
pub fn solve_part1(input: &GeneratorResult) -> usize {
    let mut next = simulate(input);
    while next.1 {
        println!("{}", next.0.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<String>>().join("\n"));
        println!();
        next = simulate(&next.0);
    }
    next = simulate(&next.0);
    next.0.iter()
        .flatten()
        .filter(|c| **c == '#')
        .count()
}

fn access2(input: &GeneratorResult, row: usize, col: usize, row_offset: isize, col_offset: isize) -> char {
    let mut c = 1;
    loop {
        let found = access(input, row, col, row_offset * c, col_offset * c);
        if found != '.' {
            break found;
        }
        c = c + 1;
    }
}

fn simulate2(input: &GeneratorResult) -> (GeneratorResult, bool) {
    let mut changed = false;
    let result: GeneratorResult = (0..input.len()).map(|row| (0..input[row].len()).map(|col| {
        let to_be = match input[row][col] {
            'L' => {
                if (-1..=1).any(|row_offset| (-1..=1).any(|col_offset| access2(input, row, col, row_offset, col_offset) == '#')) {'L'} else {'#'}
            }
            '#' => {
                if (-1..=1).map(|row_offset| (-1..=1).filter(|col_offset| access2(input, row, col, row_offset, *col_offset) == '#').count()).sum::<usize>() < 6 {'#'} else {'L'}
            }
            _ => input[row][col]
        };
        if to_be != input[row][col] {
            changed = true;
        }
        to_be
    }).collect()).collect();
    (result, changed)
}

#[aoc(day11, part2)]
pub fn solve_part2(input: &GeneratorResult) -> usize {
    let mut next = simulate(input);
    while next.1 {
        next = simulate2(&next.0);
    }
    next.0.iter()
        .flatten()
        .filter(|c| **c == '#')
        .count()
}