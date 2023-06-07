use anyhow::{anyhow, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Instruction {
    NoOp,
    Add(char, isize),
}

struct CpuEmulator {
    program_counter: usize,
    program: Vec<Instruction>,

    x_register: isize, // x register from input
    y_register: isize, // signal strength accumulator (part 1)
    z_register: isize, // I downloaded this extra ram

    breakpoints: Vec<usize>,
    debug_callback: Option<Box<dyn FnMut(&mut Self)>>,
}

impl CpuEmulator {
    fn new() -> Self {
        CpuEmulator {
            program_counter: 0,
            program: vec![],

            x_register: 0,
            y_register: 0,
            z_register: 0,

            breakpoints: vec![],
            debug_callback: None,
        }
    }

    fn execute(&mut self, program: Vec<Instruction>) {
        self.program = program;
        self.program_counter = 0;
        while self.program_counter < self.program.len() as usize {
            match self.program[self.program_counter] {
                Instruction::NoOp => self.noop(),
                Instruction::Add(register, value) => self.add(register, value),
            }
        }
    }

    fn set_breakpoints(&mut self, breakpoints: Vec<usize>) {
        self.breakpoints = breakpoints;
    }

    fn set_debug_callback(&mut self, callback: Box<dyn FnMut(&mut Self)>) {
        self.debug_callback = Some(callback);
    }

    // Step increments the CPU's program counter and optionally runs a callback if there is a breakpoint at the current program counter.
    fn step(&mut self) {
        self.program_counter += 1;
        if self.breakpoints.contains(&self.program_counter) && self.debug_callback.is_some() {
            let cb = self.debug_callback.as_mut().unwrap();
            cb(self);
        }
    }

    fn noop(&mut self) {
        self.step();
    }

    fn add(&mut self, register: char, value: isize) {
        self.step();
        self.step();
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

fn foo(cpu: &mut CpuEmulator) {
    let strength = cpu.x_register * cpu.program_counter as isize;
    cpu.y_register += strength;
    println!(
        "pc: {}, x: {}, strength {}, accumulated: {}",
        cpu.program_counter, cpu.x_register, strength, cpu.y_register
    );
}

#[aoc(day10, part1)]
#[allow(clippy::ptr_arg)]
fn solve_part1(instructions: &Vec<Instruction>) -> isize {
    let mut cpu = CpuEmulator::new();

    cpu.set_breakpoints(vec![20, 60, 100, 140, 180, 220]);
    cpu.set_debug_callback(Box::new(foo));

    cpu.execute(instructions.clone());
    0
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
