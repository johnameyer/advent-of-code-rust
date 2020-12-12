type Coord = (i32, i32);
type GeneratorResult = Vec<(char, i32)>;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            (l.chars().next().unwrap(), l.to_string()[1..].parse().unwrap())
        }).collect()
}

fn rotate(input: Coord, direction: i32) -> Coord {
    match (direction + 4) % 4 {
        0 => input,
        1 => (-input.1, input.0),
        2 => (-input.0, -input.1),
        3 => (input.1, -input.0),
        _ => panic!()
    }
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &GeneratorResult) -> i32 {
    let mut direction: i32 = 0;
    let mut coords: Coord = (0, 0);
    for (command, arg) in input {
        match command {
            'N' => coords.1 += arg,
            'S' => coords.1 -= arg,
            'E' => coords.0 += arg,
            'W' => coords.0 -= arg,
            'L' => direction += arg / 90,
            'R' => direction -= arg / 90,
            'F' => {
                let unit_vector = rotate((1,0), direction);
                coords.0 += unit_vector.0 * arg;
                coords.1 += unit_vector.1 * arg;
            }
            _ => panic!()
        }
        direction = (direction + 4) % 4;
    }
    coords.0.abs() + coords.1.abs()
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &GeneratorResult) -> i32 {
    let mut direction: i32 = 0;
    let mut waypoint: Coord = (10, 1);
    let mut coords: Coord = (0, 0);
    for (command, arg) in input {
        match command {
            'N' => waypoint.1 += arg,
            'S' => waypoint.1 -= arg,
            'E' => waypoint.0 += arg,
            'W' => waypoint.0 -= arg,
            'L' => direction += arg / 90,
            'R' => direction -= arg / 90,
            'F' => {
                coords.0 += waypoint.0 * arg;
                coords.1 += waypoint.1 * arg;
            }
            _ => panic!()
        }
        waypoint = rotate(waypoint, direction);
        direction = 0;
    }
    coords.0.abs() + coords.1.abs()
}