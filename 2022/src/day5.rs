use regex::Regex;

use crate::ASCII_UPPERCASE;

type Stack = Vec<char>;

#[derive(Debug, Clone)]
struct Harbor {
    size: usize,
    stacks: Vec<Stack>,
}

impl Harbor {
    pub fn new(size: usize) -> Self {
        Harbor {
            size,
            stacks: vec![Stack::default(); size], // n empty Stacks
        }
    }

    pub fn load_from_line(&mut self, line: &str) {
        let expected_length = (self.size * 4) - 1;
        if line.len() != (self.size * 4) - 1 {
            panic!(
                "Could not parse line: {line} (expected {expected_length} chars, found {})",
                line.len()
            );
        }
        let chars: Vec<char> = line.chars().collect();

        for i in 0..self.size {
            let position = (i * 4) + 1;
            let contents = chars[position];
            if ASCII_UPPERCASE.contains(&contents) {
                self.stacks[i].push(contents)
            } else if contents != ' ' {
                panic!("Cound not parse line: {:?} (invalid contents {contents} at position {position}", line)
            }
        }
    }

    // a Hanoi Move moves items one at a time from the source to destination stack
    pub fn perform_hanoi_move(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.quantity {
            let content = self.stacks[instruction.source].pop().unwrap();
            self.stacks[instruction.destination].push(content);
        }
    }

    // a Lift and Shift preserves the original ordering of items while moving them from the source to desination stack
    pub fn perform_lift_and_shift(&mut self, instruction: &Instruction) {
        let mut lifted = Stack::default();
        for _ in 0..instruction.quantity {
            let content = self.stacks[instruction.source].pop().unwrap();
            lifted.push(content);
        }

        while ! lifted.is_empty() {
            let content = lifted.pop().unwrap();
            self.stacks[instruction.destination].push(content);
        }
    }
}

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    source: usize,
    destination: usize,
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Harbor, Vec<Instruction>) {
    let split: Vec<&str> = input.split("\n\n").collect();
    let (harbor, instructions) = match split.len() {
        2 => (parse_harbor(split[0]), parse_instructions(split[1])),
        _ => panic!("Could not parse input: invalid sections"),
    };
    (harbor, instructions)
}

fn parse_harbor(input: &str) -> Harbor {
    let mut input = input.lines().rev();
    let stack_ids = input.next().unwrap();
    let stack_ids: Vec<&str> = stack_ids.split_whitespace().collect();
    let num_stacks: usize = stack_ids[stack_ids.len() - 1].parse::<usize>().unwrap();

    let mut harbor = Harbor::new(num_stacks);
    for line in input {
        harbor.load_from_line(line);
    }
    harbor
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    let expected_format = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = expected_format.captures(line).unwrap();
            if captures.len() != 4 {
                panic!(
                    "Could not parse instructions from line: {} ({} captures)",
                    line,
                    captures.len()
                )
            }

            Instruction {
                quantity: captures[1].parse::<usize>().unwrap(),
                source: captures[2].parse::<usize>().unwrap() - 1,
                destination: captures[3].parse::<usize>().unwrap() - 1,
            }
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day5, part1)]
fn solve_part1(input: &(Harbor, Vec<Instruction>)) -> String {
    let (harbor, instructions) = input;
    let mut harbor = harbor.clone(); // Cargo AOC only passes input as immutable, so we need to make a clone to work with

    for instruction in instructions.iter() {
        harbor.perform_hanoi_move(instruction);
    }

    let output = harbor.stacks.iter().map(|stack| {
        stack[stack.len() - 1]
    }).collect::<String>();

    output
}

#[aoc(day5, part2)]
fn solve_part2(input: &(Harbor, Vec<Instruction>)) -> String {
    let (harbor, instructions) = input;
    let mut harbor = harbor.clone(); // Cargo AOC only passes input as immutable, so we need to make a clone to work with

    for instruction in instructions.iter() {
        harbor.perform_lift_and_shift(instruction);
    }

    let output = harbor.stacks.iter().map(|stack| {
        stack[stack.len() - 1]
    }).collect::<String>();   

    output
}
