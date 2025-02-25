use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation {
    Do,
    DoNot,
}
struct Instruction {
    left: usize,
    right: usize,
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    static PATTERN: &str = r"mul\((?<left>\d{0,3}),(?<right>\d{0,3})\)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN).expect("invalid regex"));
    RE.captures_iter(input)
        .map(|c| {
            assert!(c.len() == 3);
            let left = c["left"].parse::<usize>().unwrap();
            let right = c["right"].parse::<usize>().unwrap();
            Instruction { left, right }
        })
        .collect()
}

fn run_instructions(input: Vec<Instruction>) -> usize {
    input.iter().map(|i| i.left * i.right).sum()
}

#[aoc(day3, part1)]
fn solve_part1(input: &str) -> usize {
    let input = parse_instructions(input);
    run_instructions(input)
}

fn split_instructions(input: &str) -> Vec<(Operation, &str)> {
    static PATTERN: &str = r"(?<do>don't|do)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN).expect("invalid regex"));
    let mut operations: Vec<Operation> = RE
        .captures_iter(input)
        .map(|c| {
            assert_eq!(c.len(), 2);
            match &c[1] {
                "do" => Operation::Do,
                "don't" => Operation::DoNot,
                _ => panic!("wtf?"),
            }
        })
        .collect();

    let instructions: Vec<&str> = RE.split(input).collect();

    // Some inputs may not begin with a "do" or "don't"
    // In those cases, we will have an extra set of instructions
    // Per the problem description, those initial instructions are implied to be "do"
    assert!(instructions.len() == operations.len() || instructions.len() == operations.len() + 1);
    if operations.len() != instructions.len() {
        operations.insert(0, Operation::Do);
    }

    operations.into_iter().zip(instructions).collect()
}

#[aoc(day3, part2)]
fn solve_part2(input: &str) -> usize {
    let mut total = 0;
    let input = split_instructions(input);
    for (operation, instructions) in input {
        match operation {
            Operation::Do => total += run_instructions(parse_instructions(instructions)),
            Operation::DoNot => continue,
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let test = "mul(5,123)";
        let foo = parse_instructions(test);
        assert_eq!(foo.len(), 1);
        let ins = foo.first().unwrap();
        assert_eq!(ins.left, 5);
        assert_eq!(ins.right, 123);

        let test = "mul(5,123),asdasdasdmul(0,0)";
        let foo = parse_instructions(test);
        assert_eq!(foo.len(), 2);
        let ins = foo.last().unwrap();
        assert_eq!(ins.left, 0);
        assert_eq!(ins.right, 0);
    }

    #[test]
    fn split_instruction() {
        let test = "mul(1,1)domul(5,123),mul(5,678)don'tmul(0,0)";
        let foo = split_instructions(test);
        println!("{:?}", foo);
        assert_eq!(foo.len(), 3);
        assert_eq!(foo[0].0, Operation::Do);
        assert_eq!(foo[1].0, Operation::Do);
        assert_eq!(foo[2].0, Operation::DoNot)
    }
}
