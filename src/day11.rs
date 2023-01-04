use std::collections::{HashMap, VecDeque};

use regex::Regex;

use anyhow::{anyhow, Result};

#[derive(Debug, Clone)]
enum Operation {
    Add,
    Multiply,
}

#[derive(Debug, Clone)]
enum Magnitude {
    SameAsBefore,
    Factor(usize),
}

#[derive(Debug, Clone)]
struct Thrower {
    items: VecDeque<usize>,
    operation: Operation,
    magnitude: Magnitude,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

// #[aoc_generator(day11)]
fn parse_input(input: &str) -> HashMap<usize, Thrower> {
    let mut monkeys = HashMap::<usize, Thrower>::new();
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let id = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let items = capture_numbers(lines.next().unwrap()).unwrap();
        let (operation, magnitude) = capture_operation(lines.next().unwrap()).unwrap();
        let test_value = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let true_target = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let false_target = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let monkey = Thrower {
            items: items.into_iter().collect(),
            operation,
            magnitude,
            test_value,
            true_target,
            false_target,
        };
        monkeys.insert(id, monkey);

        let _ = lines.next(); // Skip the blank line separating input blocks
    }

    monkeys
}

#[aoc(day11, part1)]
fn solve_part1(input: &str) -> usize {
    let mut monkeys = parse_input(input);
    run_round(&mut monkeys);
    0
}

fn run_round(monkeys: &mut HashMap<usize, Thrower>) {
    println!("Round start {:?}", monkeys);
    for i in 0..monkeys.len() {
        run_turn(i, monkeys)
    }
    println!("Round end {:?}", monkeys);
}

fn run_turn(id: usize, monkeys: &mut HashMap<usize, Thrower>) {
    let mut updates = VecDeque::<(usize, usize)>::new();
    let monkey = monkeys.get_mut(&id).unwrap();
    while !monkey.items.is_empty() {
        let item_score = monkey.items.pop_front().unwrap();
        let item_score = match monkey.operation {
            Operation::Add => match monkey.magnitude {
                Magnitude::Factor(n) => item_score + n,
                Magnitude::SameAsBefore => item_score + item_score,
            },
            Operation::Multiply => match monkey.magnitude {
                Magnitude::Factor(n) => item_score * n,
                Magnitude::SameAsBefore => item_score * item_score,
            },
        };
        let item_score = (item_score as f64 / 3.0).floor() as usize;

        let target = match item_score % monkey.test_value == 0 {
            true => monkey.true_target,
            false => monkey.false_target,
        };

        updates.push_back((target, item_score));
    }

    while !updates.is_empty() {
        let (target, item_score) = updates.pop_front().unwrap();
        let monkey = monkeys.get_mut(&target).unwrap();
        monkey.items.push_back(item_score);
    }
}

fn capture_numbers(line: &str) -> Result<Vec<usize>> {
    let re = Regex::new(r"(\d+)").unwrap();
    let items: Vec<usize> = re
        .captures_iter(line)
        .filter_map(|captures| captures[1].parse::<usize>().ok())
        .collect();

    Ok(items)
}

fn capture_operation(line: &str) -> Result<(Operation, Magnitude)> {
    let re = Regex::new(r"(\*|\+) (old|\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    println!("parsing op from line {}: {:?}", line, captures);

    if captures.len() != 3 {
        return Err(anyhow!(
            "Could not parse operation: wrong number of captures found"
        ));
    }

    let operation = match &captures[1] {
        "+" => Operation::Add,
        "*" => Operation::Multiply,
        _ => return Err(anyhow!("Could not parse operation: invalid symbol")),
    };

    let magnitude = if &captures[2] == "old" {
        Magnitude::SameAsBefore
    } else if captures[2].parse::<usize>().is_ok() {
        let force = captures[2].parse::<usize>().unwrap();
        Magnitude::Factor(force)
    } else {
        return Err(anyhow!("could not parse operation: invalid magnitude"));
    };

    Ok((operation, magnitude))
}
