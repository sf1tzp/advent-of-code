use std::collections::VecDeque;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    NoOp,
    Add(char, isize),
}

struct CpuEmulator {
    clock_counter: usize,

    // Registers
    x_register: isize,
    y_register: isize,
    z_register: isize,

    // Program memory
    program: VecDeque<Instruction>,
    program_counter: usize,

    // Debugging features!
    // Run code at a specific clock tick or program step
    checkpoints: Vec<usize>, // Clock ticks at which to run the debug callback
    clock_debug_callback: fn(&mut Self),

    breakpoints: Vec<usize>, // Program steps at which to run the debug callback
    program_debug_callback: fn(&mut Self),
}

impl CpuEmulator {
    fn new() -> Self {
        CpuEmulator {
            clock_counter: 0,

            x_register: 0,
            y_register: 0,
            z_register: 0,

            program_counter: 0,
            program: VecDeque::new(),

            checkpoints: vec![],
            clock_debug_callback: |_| {},
            breakpoints: vec![],
            program_debug_callback: |_| {},
        }
    }

    // Step increments the CPU's program counter and
    // optionally runs a callback if there is a breakpoint at the current program counter.
    fn clock_tick(&mut self) {
        self.clock_counter += 1;

        if self.checkpoints.contains(&self.clock_counter) {
            (self.clock_debug_callback)(self);
        }
    }

    fn run(&mut self, program: &[Instruction]) {
        self.program_counter = 0;
        self.program = VecDeque::from(program.to_owned());

        // Pop instructions off the program memory and execute them until the program memory is empty.
        // This avoids having to borrow self mutably twice (ie, once with iter and then with the operations)
        while let Some(i) = self.program.pop_front() {
            self.program_counter += 1;

            if self.breakpoints.contains(&self.program_counter) {
                (self.program_debug_callback)(self);
            }

            match i {
                Instruction::NoOp => self.noop(),
                Instruction::Add(register, value) => self.add(register, value),
            }
        }
    }

    fn noop(&mut self) {
        self.clock_tick();
    }

    fn add(&mut self, register: char, value: isize) {
        self.clock_tick();
        self.clock_tick();
        match register {
            'x' => self.x_register += value,
            'y' => self.y_register += value,
            'z' => self.z_register += value,
            _ => panic!("Invalid register: {}", register),
        }
    }
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
fn solve_part1(program: &[Instruction]) -> isize {
    let mut cpu = CpuEmulator::new();

    cpu.x_register = 1; // Starts at one per the problem statement

    cpu.checkpoints = vec![20, 60, 100, 140, 180, 220]; // Clock ticks at which to check the signal
    cpu.clock_debug_callback = |cpu| {
        let strength = cpu.x_register * cpu.clock_counter as isize;
        cpu.y_register += strength;
    };

    // We could also run steps at specific program counters instead of clock ticks
    // Eg, Display the state of the CPU at the end of the program
    // cpu.breakpoints = vec![program.len()];
    // cpu.program_debug_callback = |cpu| {
    //     println!("Program counter: {}", cpu.program_counter);
    //     println!("X register: {}", cpu.x_register);
    //     println!("Y register: {}", cpu.y_register);
    //     println!("Z register: {}", cpu.z_register);
    // };

    cpu.run(program);

    cpu.y_register
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
