pub struct Password {
    min: usize,
    max: usize,
    character: char,
    password: String
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let mut iterator = l.trim().split(&['-', ' ', ':'] as &[char]).filter(|s| s.len() > 0 );
            Password {
                min: iterator.next().unwrap().parse().unwrap(),
                max: iterator.next().unwrap().parse().unwrap(),
                character: iterator.next().unwrap().chars().next().unwrap(),
                password: iterator.next().unwrap().into()
            }
        }).collect::<Vec<Password>>()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Password>) -> u32 {
    // O(n*l)
    input.into_iter().fold(0 as u32, |count, password| {
        let letter_count =  password.password.chars().fold(0, |count, character| {
                count + if character == password.character { 1 } else { 0 }
        });
        count + if password.min <= letter_count && letter_count <= password.max { 1 } else { 0 }
    })
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Password>) -> u32 {
    // O(n*l)
    input.into_iter().fold(0 as u32, |count, password| {
        let first_matches = password.password.chars().nth(password.min - 1) == Some(password.character);
        let second_matches = password.password.chars().nth(password.max - 1) == Some(password.character);
        count + if first_matches != second_matches { 1 } else { 0 }
    })
}