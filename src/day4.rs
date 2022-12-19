use anyhow::{anyhow, Result};

#[derive(Debug)]
struct SectionID {
    start: usize,
    end: usize,
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> Vec<(SectionID, SectionID)> {
    input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            let (id1, id2) = match split.len() {
                2 => (parse_id(split[0]).unwrap(), parse_id(split[1]).unwrap()),
                _ => panic!("Could not parse line {}", line), // better way to error from within map?
            };
            (id1, id2)
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[(SectionID, SectionID)]) -> usize {
    input
        .iter()
        .map(|ids| check_complete_overlap(&ids.0, &ids.1))
        .map(|yes| if yes { 1 } else { 0 })
        .sum()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[(SectionID, SectionID)]) -> usize {
    input
        .iter()
        .map(|ids| check_partial_overlap(&ids.0, &ids.1))
        .map(|yes| if yes { 1 } else { 0 })
        .sum()
}

fn parse_id(id: &str) -> anyhow::Result<SectionID> {
    let split: Vec<&str> = id.split("-").collect();
    let (start, end) = match split.len() {
        2 => (
            split[0].parse::<usize>().unwrap(),
            split[1].parse::<usize>().unwrap(),
        ),
        _ => return Err(anyhow!("Could not parse ID {}", id)),
    };

    Ok(SectionID { start, end })
}

fn check_complete_overlap(id1: &SectionID, id2: &SectionID) -> bool {
    if id1.start <= id2.start && id1.end >= id2.end {
        true
    } else if id2.start <= id1.start && id2.end >= id1.end {
        true
    } else {
        false
    }
}

fn check_partial_overlap(id1: &SectionID, id2: &SectionID) -> bool {
    if id1.start <= id2.end && id2.start <= id1.end {
        true
    } else if id2.start <= id1.end && id1.start <= id2.end {
        true
    } else {
        false
    }
}
