enum Direction {
    FORWARD,
    DOWN,
    UP,
}

impl Direction {
    fn from(str: &str) -> Direction {
        match str {
            "forward" => Direction::FORWARD,
            "down" => Direction::DOWN,
            "up" => Direction::UP,
            _ => panic!("Invalid direction"),
        }
    }
}

pub struct Command {
    direction: Direction,
    distance: u32,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|l| l.trim().split(' '))
        .map(|mut l| Command {
            direction: Direction::from(l.next().unwrap()),
            distance: l.next().unwrap().parse().unwrap(),
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &Vec<Command>) -> u32 {
    let result: (u32, u32) = input.iter().fold((0, 0), |acc, i| match &i.direction {
        Direction::FORWARD => (acc.0, acc.1 + i.distance),
        Direction::UP => (acc.0 - i.distance, acc.1),
        Direction::DOWN => (acc.0 + i.distance, acc.1),
    });
    result.0 * result.1
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &Vec<Command>) -> u32 {
    let result: (u32, u32, u32) = input.iter().fold((0, 0, 0), |acc, i| match &i.direction {
        Direction::DOWN => (acc.0, acc.1, acc.2 + i.distance),
        Direction::UP => (acc.0, acc.1, acc.2 - i.distance),
        Direction::FORWARD => (acc.0 + i.distance, acc.1 + i.distance * acc.2, acc.2),
    });
    result.0 * result.1
}
