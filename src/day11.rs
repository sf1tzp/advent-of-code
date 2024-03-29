use std::collections::{HashMap, VecDeque};

use regex::Regex;

use anyhow::{anyhow, Result};
use itertools::Itertools; // itertools = "0.8"

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
struct Monkey {
    items: VecDeque<usize>,
    operation: Operation,
    magnitude: Magnitude,
    test_value: usize,
    true_target: usize,
    false_target: usize,
}

impl Monkey {
    fn run_turn(&mut self, worry_factor: Option<f64>, lcm: usize) -> VecDeque<(usize, usize)> {
        let mut item_updates = VecDeque::<(usize, usize)>::new();
        while !self.items.is_empty() {
            let item_score = self.items.pop_front().unwrap();
            let item_score = match self.operation {
                Operation::Add => match self.magnitude {
                    Magnitude::Factor(n) => item_score + n,
                    Magnitude::SameAsBefore => item_score + item_score,
                },
                Operation::Multiply => match self.magnitude {
                    Magnitude::Factor(n) => item_score * n,
                    Magnitude::SameAsBefore => item_score * item_score,
                },
            };

            let item_score = match worry_factor {
                Some(n) => (item_score as f64 / n).floor() as usize,
                None => {
                    // Item score may be huge, so mod it against the lcm
                    // This still allows us to select the correct target
                    // because the lcm is a multiple of the test value
                    item_score % lcm
                },
            };

            let target = match item_score % self.test_value == 0 {
                true => self.true_target,
                false => self.false_target,
            };

            item_updates.push_back((target, item_score));
        }
        item_updates
    }
}

#[derive(Debug, Clone)]
struct MonkeyBusinessTracker {
    monkeys: HashMap<usize, Monkey>,
    inspection_counts: HashMap<usize, usize>,
}

impl MonkeyBusinessTracker {
    fn new_from_input(monkeys: HashMap<usize, Monkey>) -> Self {
        MonkeyBusinessTracker {
            monkeys,
            inspection_counts: HashMap::<usize, usize>::new(),
        }
    }

    fn update_activity(&mut self, monkey_id: usize, count: usize) {
        self.inspection_counts
            .entry(monkey_id)
            .and_modify(|counter| *counter += count)
            .or_insert(count);
    }

    fn update_items(&mut self, updates: VecDeque<(usize, usize)>) {
        for (monkey_id, item_score) in updates {
            let target_monkey = self.monkeys.get_mut(&monkey_id).unwrap();
            target_monkey.items.push_back(item_score);
        }
    }

    #[allow(dead_code)]
    fn print_items(&self) {
        for (monkey_id, monkey) in self.monkeys.iter() {
            println!("Monkey {}: {:?}", monkey_id, monkey.items);
        }
    }

    #[allow(dead_code)]
    fn print_inspections(&self) {
        for monkey_id in self.inspection_counts.keys().sorted() {
            println!(
                "Monkey {} has inspected {:?} items",
                monkey_id,
                self.inspection_counts.get(monkey_id).unwrap()
            );
        }
    }

    fn calculate(&self, n: usize) -> usize {
        // Find the ids of the top two inspection counts
        self.inspection_counts
            .values()
            .sorted()
            .rev()
            .take(n)
            .copied()
            .product()
    }

    // Calculate the least common multiple of all the test values
    // This allows us to select targets without worrying about overflow
    fn get_lcm(&self) -> usize {
        self.monkeys
            .values()
            .map(|monkey| monkey.test_value)
            .product()
    }
}

// Parse a list of numbers from a string
fn capture_numbers(line: &str) -> Result<Vec<usize>> {
    let re = Regex::new(r"(\d+)").unwrap();
    let items: Vec<usize> = re
        .captures_iter(line)
        .filter_map(|captures| captures[1].parse::<usize>().ok())
        .collect();

    Ok(items)
}

// Parse an Operation from a string
fn capture_operation(line: &str) -> Result<(Operation, Magnitude)> {
    let re = Regex::new(r"(\*|\+) (old|\d+)").unwrap();
    let captures = re.captures(line).unwrap();
    // println!("parsing op from line {}: {:?}", line, captures);

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

#[aoc_generator(day11)]
fn parse_input(input: &str) -> HashMap<usize, Monkey> {
    let mut monkeys = HashMap::<usize, Monkey>::new();
    let mut lines = input.lines().peekable();

    while lines.peek().is_some() {
        let id = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let items = capture_numbers(lines.next().unwrap()).unwrap();
        let (operation, magnitude) = capture_operation(lines.next().unwrap()).unwrap();
        let test_value = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let true_target = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let false_target = capture_numbers(lines.next().unwrap()).unwrap()[0];
        let monkey = Monkey {
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
fn solve_part1(input: &HashMap<usize, Monkey>) -> usize {
    let rounds = 20;
    let worry_factor = Some(3.0);
    let monkey_business_factor = 2;

    let mut tracker = MonkeyBusinessTracker::new_from_input(input.clone());
    let lcm = tracker.get_lcm();
    // tracker.print_items();
    for _ in 0..rounds {
        // println!("Round {}", i + 1);
        for monkey_id in 0..tracker.monkeys.len() {
            let item_count = tracker.monkeys.get(&monkey_id).unwrap().items.len();
            tracker.update_activity(monkey_id, item_count);
            let monkey = tracker.monkeys.get_mut(&monkey_id).unwrap();
            let updates = monkey.run_turn(worry_factor, lcm);
            tracker.update_items(updates);
        }
        // tracker.print_items();
    }
    // tracker.print_inspections();
    tracker.calculate(monkey_business_factor)
}

#[aoc(day11, part2)]
fn solve_part2(input: &HashMap<usize, Monkey>) -> usize {
    let rounds = 10_000;
    let worry_factor = None;
    let monkey_business_factor = 2;
    let debug_rounds: Vec<i32> = vec![
        1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
    ];

    let mut tracker = MonkeyBusinessTracker::new_from_input(input.clone());
    let lcm = tracker.get_lcm();
    for i in 0..rounds {
        for monkey_id in 0..tracker.monkeys.len() {
            let item_count = tracker.monkeys.get(&monkey_id).unwrap().items.len();
            tracker.update_activity(monkey_id, item_count);
            let monkey = tracker.monkeys.get_mut(&monkey_id).unwrap();
            let updates = monkey.run_turn(worry_factor, lcm);
            tracker.update_items(updates);
        }
        if debug_rounds.contains(&(i + 1)) {
            // println!("Round {}", i + 1);
            // tracker.print_items();
            // tracker.print_inspections();
        }
    }
    tracker.calculate(monkey_business_factor)
}
