type GeneratorResult = (u64, Vec<u64>);

#[aoc_generator(day13, part1)]
pub fn input_generator(input: &str) -> GeneratorResult {
    let lines = input
        .split("\n").collect::<Vec<&str>>();

    let early = lines[0].parse().unwrap();
    let busses: Vec<u64> = lines[1]
        .split(",")
        .filter(|&id| id != "x")
        .map(|id| {
            id.parse().unwrap()
        }).collect();
    (early, busses)
}

fn next_after(time: u64, id: u64) -> u64 {
    let check = time / id;
    if check * id > time {
        check * id
    } else {
        ( check +1 )* id
    }
}

#[aoc(day13, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    let soonest = input.1.iter()
        .min_by_key(|&&id| next_after(input.0, id))
        .unwrap();
    (soonest) * (next_after(input.0, *soonest) - input.0)
}


#[aoc_generator(day13, part2)]
pub fn input_generator2(input: &str) -> GeneratorResult {
    let lines = input
        .split("\n").collect::<Vec<&str>>();

    let early = lines[0].parse().unwrap();
    let busses: Vec<u64> = lines[1]
        .split(",")
        .map(|id| {
            if id != "x" {
            id.parse().unwrap()
            } else {
                0
            }
        }).collect();
    (early, busses)
}


#[aoc(day13, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let chinese_remainder_theorem_coefficients = input.1.iter().enumerate().filter(|(_list_offset, id)| **id != 0).map(|(list_offset, id)| (*id - (list_offset as u64 % *id), *id)).collect::<Vec<(u64, u64)>>();
    println!("At https://www.dcode.fr/chinese-remainder your coeffs are {:?}", chinese_remainder_theorem_coefficients);
    // Fallback to optimized naive computation since not sure of how to calculate above solution
    let mut counter = 0;
    let max = input.1.iter().enumerate().max_by_key(|(_, item)| **item).unwrap();
    loop {
        if input.1.iter().enumerate()
            .filter(|(_list_offset, id)| **id != 0)
            .all(|(list_offset, id)| ((counter as i64 + (list_offset as i64 - max.0 as i64)) as u64) % id == 0) {
            break counter;
        }
        counter += max.1;
    }
}