use anyhow::{anyhow, Result};
use once_cell::sync::Lazy;
use regex::Regex;

enum Operation {
    Multiply,
}
struct Instruction {
    operation: Operation,
    left: usize,
    right: usize,
}

#[aoc_generator(day3)]
fn parse_instructions(input: &str) -> Vec<Instruction> {
    static PATTERN: &str = r"mul\((?<left>\d{0,3}),(?<right>\d{0,3})\)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN).expect("invalid regex"));
    RE.captures_iter(input)
        .map(|c| {
            assert!(c.len() == 3);
            let left = c["left"].parse::<usize>().unwrap();
            let right = c["right"].parse::<usize>().unwrap();
            Instruction {
                operation: Operation::Multiply,
                left,
                right,
            }
        })
        .collect()
}

#[aoc(day3, part1)]
fn solve_part1(input: &Vec<Instruction>) -> usize {
    input.iter().map(|i| i.left * i.right).sum()
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
}
