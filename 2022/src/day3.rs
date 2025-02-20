use std::collections::HashSet;

use anyhow::anyhow;

use crate::{ASCII_LOWERCASE, ASCII_UPPERCASE};

type Item = char;
type Priority = usize;

#[derive(Debug)]
struct Backpack {
    compartment1: Vec<Item>,
    compartment2: Vec<Item>,
}

impl Backpack {
    fn all(&self) -> Vec<Item> {
        self.compartment1
            .iter()
            .cloned()
            .chain(self.compartment2.iter().cloned())
            .collect()
    }
}

#[aoc_generator(day3)]
fn parse_input(input: &str) -> Vec<Backpack> {
    let mut backpacks = Vec::<Backpack>::new();
    for (i, line) in input.split('\n').enumerate() {
        if line.len() % 2 != 0 {
            panic!("Line {i} is not even: {line}");
        }
        let divider = line.len() / 2;

        let backpack = Backpack {
            compartment1: line[..divider].chars().collect(),
            compartment2: line[divider..].chars().collect(),
        };

        backpacks.push(backpack);
    }
    backpacks
}

#[aoc(day3, part1)]
fn solve_part1(input: &[Backpack]) -> Priority {
    input
        .iter()
        .map(|backpack| {
            let misplaced = find_misplaced_items(backpack).unwrap();
            check_priority(misplaced[0]).unwrap()
        })
        .sum()
}

#[aoc(day3, part2)]
fn solve_part2(input: &[Backpack]) -> Priority {
    if input.len() % 3 != 0 {
        panic!("{} is not divisile by 3, cannot proceed", input.len());
    }

    let mut priorities = 0;
    for i in 0..input.len() / 3 {
        let j = i * 3;
        let backpack1 = &input[j];
        let backpack2 = &input[j + 1];
        let backpack3 = &input[j + 2];

        let badge = find_badge_item(backpack1, backpack2, backpack3).unwrap();
        priorities += check_priority(badge).unwrap();
    }
    priorities
}

fn check_priority(item: char) -> anyhow::Result<Priority> {
    if ASCII_LOWERCASE.contains(&item) {
        Ok(item as usize - 96)
    } else if ASCII_UPPERCASE.contains(&item) {
        Ok(item as usize - 38)
    } else {
        Err(anyhow!("invalid item"))
    }
}

fn find_misplaced_items(backpack: &Backpack) -> Option<Vec<Item>> {
    let compartment1 = HashSet::<Item>::from_iter(backpack.compartment1.clone());
    let compartment2 = HashSet::<Item>::from_iter(backpack.compartment2.clone());

    let intersection: Vec<Item> = compartment1.intersection(&compartment2).cloned().collect();

    match intersection.len() {
        0 => None,
        1 => Some(intersection),
        _ => {
            println!("More than one misplaced item found {:?}", intersection);
            None
        }
    }
}

fn find_badge_item(
    backpack1: &Backpack,
    backpack2: &Backpack,
    backpack3: &Backpack,
) -> Option<Item> {
    let items1 = HashSet::<Item>::from_iter(backpack1.all());
    let items2 = HashSet::<Item>::from_iter(backpack2.all());
    let items3 = HashSet::<Item>::from_iter(backpack3.all());

    let intersection = HashSet::<Item>::from_iter(items1.intersection(&items2).cloned());
    let intersection: Vec<Item> = intersection.intersection(&items3).cloned().collect();

    match intersection.len() {
        0 => None,
        1 => Some(intersection[0]),
        _ => {
            println!("More than one badge found {:?}", intersection);
            None
        }
    }
}
