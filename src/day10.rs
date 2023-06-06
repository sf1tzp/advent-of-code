use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    NoOp,
    Add(char, isize),
}

struct Cpu {
    program_counter: isize,
    x_register: isize,
}

fn parse_instruction(instruction: &str) -> Result<Instruction> {
    let parts: Vec<&str> = instruction.split_whitespace().collect();
    // noop => NoOp
    // add* => Add('*', 0) where * could be any character in lower ASCII
    match parts.len() {
        1 if parts[0] == "noop" => Ok(Instruction::NoOp),
        2 if &parts[0][0..3] == "add" && parts[0].len() == 4 => {
            let register = parts[0][3..4]
                .chars()
                .next()
                .ok_or_else(|| anyhow!("Invalid register: {}", parts[0]));
            let value = parts[1]
                .parse::<isize>()
                .map_err(|_| anyhow!("Invalid value: {}", parts[1]));

            match (register, value) {
                (Ok(register), Ok(value)) => Ok(Instruction::Add(register, value)),
                _ => Err(anyhow!("Invalid instruction: {}", instruction)),
            }
        }
        _ => Err(anyhow!("Invalid instruction: {}", instruction)),
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| match parse_instruction(line) {
            Ok(instruction) => instruction,
            Err(err) => panic!("Error parsing instruction: {}", err),
        })
        .collect::<Vec<Instruction>>()
}

#[aoc(day10, part1)]
#[allow(clippy::ptr_arg)]
fn solve_part1(instructions: &Vec<Instruction>) -> isize {
    let mut cpu = Cpu {
        program_counter: 0,
        x_register: 1,
    };
    let mut signal_strength_sum = 0;

    fn debug_signal(cpu: &Cpu) -> isize {
        let checkpoints = vec![20, 60, 100, 140, 180, 220];
        if checkpoints.contains(&cpu.program_counter) {
            println!("pc: {}, x: {}", cpu.program_counter, cpu.x_register);
            return cpu.x_register * cpu.program_counter;
        }
        0
    }

    for instruction in instructions {
        cpu.program_counter += 1;
        signal_strength_sum += debug_signal(&cpu);

        match instruction {
            Instruction::NoOp => {},
            Instruction::Add(_, value) => {
                cpu.x_register += value;
                cpu.program_counter += 1;
                signal_strength_sum += debug_signal(&cpu);
            }
        }
    }

    signal_strength_sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_instructrion() {
        struct TestCase {
            description: &'static str,
            input: &'static str,
            expected: Result<Instruction>,
        }
        let cases: Vec<TestCase> = vec![
            TestCase {
                description: "noop",
                input: "noop",
                expected: Ok(Instruction::NoOp),
            },
            TestCase {
                description: "valid positive add",
                input: "adda 1",
                expected: Ok(Instruction::Add('a', 1)),
            },
            TestCase {
                description: "valid negative add",
                input: "addb -1",
                expected: Ok(Instruction::Add('b', -1)),
            },
            TestCase {
                description: "valid zero add",
                input: "addc 0",
                expected: Ok(Instruction::Add('c', 0)),
            },
            TestCase {
                description: "invalid instruction",
                input: "add 1",
                expected: Err(anyhow!("Invalid instruction: add 1")),
            },
        ];

        for case in cases {
            let got = parse_instruction(case.input);
            match (got, case.expected) {
                (Ok(got), Ok(expected)) => assert_eq!(got, expected),
                (Err(_), Ok(_)) => panic!("{}: got error, want no error", case.description),
                (Ok(_), Err(_)) => panic!("{}: got no error, want error", case.description),
                (Err(_), Err(_)) => {}
            }
        }
    }
}
