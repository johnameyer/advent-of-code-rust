pub enum Expression {
    VALUE(u64),
    ADD(Box<Expression>, Box<Expression>),
    SUBTRACT(Box<Expression>, Box<Expression>),
    MULTIPLY(Box<Expression>, Box<Expression>),
    DIVIDE(Box<Expression>, Box<Expression>),
    GROUP(Box<Expression>)
}

type GeneratorResult = Vec<Box<Expression>>;

impl Expression {
    fn eval(&self) -> u64 {
        match self {
            Expression::VALUE(val) => *val,
            Expression::ADD(left, right) => left.eval() + right.eval(),
            Expression::SUBTRACT(left, right) => left.eval() - right.eval(),
            Expression::MULTIPLY(left, right) => left.eval() * right.eval(),
            Expression::DIVIDE(left, right) => left.eval() / right.eval(),
            Expression::GROUP(left) => left.eval()
        }
    }

    
    fn parse(str: &str, operator_indexer: fn(str: &str) -> Option<usize>) -> Box<Expression> {
        let operator_index = operator_indexer(str);
        let parse = |str| Expression::parse(str, operator_indexer);
        Box::new(if !str.contains(" ") {
            Expression::VALUE(str.parse().unwrap())
        } else if operator_index.is_none() {
            Expression::GROUP(parse(&str[1..str.len() - 1]))
        } else {
            let (left, operator, right) = {
                (str[0..operator_index.unwrap()].trim(), &str[operator_index.unwrap()..=operator_index.unwrap()], str[(operator_index.unwrap()+1)..].trim())
            };
            match operator {
                "+" => Expression::ADD(parse(left), parse(right)),
                "-" => Expression::SUBTRACT(parse(left), parse(right)),
                "*" => Expression::MULTIPLY(parse(left), parse(right)),
                "/" => Expression::DIVIDE(parse(left), parse(right)),
                _ => panic!()
            }
        })
    }
}

fn find_operator_index(str: &str) -> Option<usize> {
    let mut counter = 0;
    for (i, c) in str.chars().rev().enumerate() {
        counter += match c {
            '(' => -1,
            ')' => 1,
            _ => 0
        };
        if counter == 0 && (c == '+' || c == '-' || c == '*' || c == '/') {
            return Some(str.len() - i - 1);
        }
    };
    None
}

#[aoc_generator(day18, part1)]
pub fn input_generator(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            Expression::parse(l, find_operator_index)
        }).collect()
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &GeneratorResult) -> u64 {
    input.iter()
        .map(|key| key.eval())
        .sum()
}

fn find_operator_index_priority(str: &str) -> Option<usize> {
    let mut counter = 0;
    let mut first = None;
    for (i, c) in str.chars().rev().enumerate() {
        counter += match c {
            '(' => -1,
            ')' => 1,
            _ => 0
        };
        if counter == 0 {
            if c == '+' || c == '-' {
                first = Some(str.len() - i - 1);
            } else if c == '*' || c == '/' {
                return Some(str.len() - i - 1);
            }
        }
    };

    first
}

#[aoc_generator(day18, part2)]
pub fn input_generator2(input: &str) -> GeneratorResult {
    input
        .split("\n")
        .map(|l| {
            Expression::parse(l, find_operator_index_priority)
        }).collect()
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &GeneratorResult) -> u64 {
    input.iter()
        .map(|key| key.eval())
        .sum()
}