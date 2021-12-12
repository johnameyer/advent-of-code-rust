use std::collections::HashMap;
use std::collections::HashSet;

#[aoc_generator(day12)]
pub fn input_generator(input: &str) -> HashMap<String, HashSet<String>> {
    input
        .lines()
        .map(|l| {
            let mut iter = l.trim()
            .split("-");
            (iter.next().unwrap().to_string(), iter.next().unwrap().to_string())
        })
        .flat_map(move |(one, two)| [(one.clone(), two.clone()), (two, one)])
        .fold(HashMap::new(), |mut map, (one, two)| {
            map.entry(one).or_insert(HashSet::new()).insert(two);
            map
        })
}

fn recurse(point: &String, input: &HashMap<String, HashSet<String>>, visited: &HashSet<String>, visited_twice: bool) -> u32 {
    if point == "end" {
        return 1;
    }
    let mut next_set: HashSet<String> = visited.iter().cloned().collect();
    next_set.insert(point.clone());
    let mut i = 0;
    for next in input.get(point).unwrap() {
        if next.to_uppercase() == *next || !next_set.contains(next) {
            i += recurse(next, input, &next_set, visited_twice);
        } else if next != "start" && !visited_twice {
            i += recurse(next, input, &next_set, true);
        }
    }
    return i;
}

#[aoc(day12, part1)]
pub fn solve_part1(input: &HashMap<String, HashSet<String>> ) -> u32 {
    recurse(&"start".to_string(), input, &HashSet::new(), true)
}

#[aoc(day12, part2)]
pub fn solve_part2(input: &HashMap<String, HashSet<String>> ) -> u32 {
    recurse(&"start".to_string(), input, &HashSet::new(), false)
}
