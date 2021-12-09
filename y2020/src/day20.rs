
use std::collections::HashSet;
use std::collections::HashMap;
use std::ops::Sub;

pub struct Tile (
    Vec<Vec<bool>>
);

fn flip(num: u64, len: usize) -> u64 {
    let mut old = num;
    let mut result = 0;
    for _ in 0..len {
        result = result << 1 | old & 1;
        old = old >> 1;
    }
    result
}

impl Tile {
    fn left(&self) -> u64 {
        self.0.iter().map(|vec| vec[0]).fold(0, |acc, current| (acc << 1) + current as u64)
    }

    fn right(&self) -> u64 {
        self.0.iter().map(|vec| vec[vec.len() - 1]).fold(0, |acc, current| (acc << 1) + current as u64)
    }

    fn top(&self) -> u64 {
        self.0[0].iter().fold(0, |acc, current| (acc << 1) + *current as u64)
    }

    fn bottom(&self) -> u64 {
        self.0[self.0.len() - 1].iter().fold(0, |acc, current| (acc << 1) + *current as u64)
    }

    fn all(&self) -> HashSet<u64> {
        vec![self.top(), self.bottom(), self.left(), self.right()].into_iter().collect()
    }

    fn all_flipped(&self) -> HashSet<u64> {
        self.all().iter().flat_map(|num| vec![*num, flip(*num, self.0.len())]).collect()
    }

    fn trim(&self) -> Vec<Vec<bool>> {
        self.0[1..(self.0.len() - 1)].iter().map(|vec| vec[1..vec.len() - 1].iter().map(|b| *b).collect()).collect()
    }
}

type GeneratorResult = HashMap<u64, Tile>;
#[aoc_generator(day20)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n\n")
        .map(|l| {
            let mut split = l.split("\n");
            let first = split.next().unwrap();
            (first[5..(first.len()-1)].parse().unwrap(), Tile(split.map(|l| l.chars().map(|c| c == '#').collect()).collect()))
        }).collect()
}

#[aoc(day20, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    let mut sides: HashMap<u64, Vec<u64>> = HashMap::new();

    for (key, tile) in input {
        for num in tile.all_flipped() {
            sides.entry(num).or_insert(vec![]).push(*key);
        }
    }

    let flip_len = input.values().next().unwrap().0.len();

    let corners: HashSet<u64> = input.iter().filter(|(key, value)| value.all().iter().filter(|num| sides[num].len() == 1 && sides[&flip(**num, flip_len)].len() == 1).count() == 2).map(|(key, value)| *key).collect();

    corners.iter().product()
}

fn insert_next_from(arranged: &mut HashMap<(u64,u64), u64>, input: &GeneratorResult, row: u64, col: u64, sides: &HashMap<u64, HashSet<u64>>, limited: &HashSet<u64>) {
    let neighbors: Vec<(u64, u64)> = vec![(row - 1, col), (row, col - 1), (row + 1, col), (row, col + 1)];
    arranged.insert((row, col), neighbors.iter()
        .filter_map(|neighbor| arranged.get(neighbor))
        .flat_map(|i| input[i].all_flipped().iter()
            .filter_map(|num| sides[num]
                .sub(&arranged.values()
                .map(|i| *i).collect()).iter()
                .map(|i| *i).next()
            ).collect::<HashSet<u64>>()
        ).filter(|i| limited.contains(i))
        .next().unwrap()
    );
}

#[aoc(day20, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    let mut sides: HashMap<u64, HashSet<u64>> = HashMap::new();

    for (key, tile) in input {
        for num in tile.all_flipped() {
            sides.entry(num).or_insert(HashSet::new()).insert(*key);
        }
    }

    let flip_len = input.values().next().unwrap().0.len();

    let corners: HashSet<u64> = input.iter().filter(|(key, value)| value.all().iter().filter(|num| sides[num].len() == 1 && sides[&flip(**num, flip_len)].len() == 1).count() == 2).map(|(key, value)| *key).collect();
    let edges: HashSet<u64> = input.iter().filter(|(key, value)| value.all().iter().filter(|num| sides[num].len() == 1 && sides[&flip(**num, flip_len)].len() == 1).count() == 1).map(|(key, value)| *key).collect();

    let mut arranged: HashMap<(u64,u64), u64> = HashMap::new();
    arranged.insert((0, 0), *corners.iter().next().unwrap());
    
    for row in 1..(flip_len-1) {
        insert_next_from(&mut arranged, input, row as u64, 0, &sides, &edges);
    }

    // bottom left corner

    for col in 1..(flip_len-1) {
        insert_next_from(&mut arranged, input, 0, col as u64, &sides, &edges);
    }

    // top right corner
    // right edge
    // bottom edge
    // bottom right corner
    // interior
    
    // flips and combine

    0
}