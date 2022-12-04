use anyhow::{anyhow, Result};

#[derive(Debug)]
struct SectionID {
    start: usize,
    end: usize,
}

fn main() -> Result<()> {
    let input = advent_of_code_2022::read_input().unwrap();
    let ids = parse_input(input).unwrap();

    let complete_overlaps: usize = ids
        .iter()
        .map(|ids| check_complete_overlap(&ids.0, &ids.1))
        .map(|yes| if yes { 1 } else { 0 })
        .sum();

    println!("(#1) Found {complete_overlaps} Completely Overlapping Pairs");

    let partial_overlaps: usize = ids
        .iter()
        .map(|ids| check_partial_overlap(&ids.0, &ids.1))
        .map(|yes| if yes { 1 } else { 0 })
        .sum();

    println!("(#2) Found {partial_overlaps} Partially Overlapping Pairs");
    Ok(())
}

fn parse_input(input: String) -> Result<Vec<(SectionID, SectionID)>> {
    let ids: Vec<(SectionID, SectionID)> = input
        .lines()
        .map(|line| {
            let split: Vec<&str> = line.split(",").collect();
            let (id1, id2) = match split.len() {
                2 => (parse_id(split[0]).unwrap(), parse_id(split[1]).unwrap()),
                _ => panic!("Could not parse line {}", line), // better way to error from within map?
            };
            (id1, id2)
        })
        .collect();

    Ok(ids)
}

fn parse_id(id: &str) -> Result<SectionID> {
    let split: Vec<&str> = id.split("-").collect();
    let (start, end) = match split.len() {
        2 => (
            split[0].parse::<usize>().unwrap(),
            split[1].parse::<usize>().unwrap(),
        ),
        _ => return Err(anyhow!("Could not parse ID {}", id)),
    };

    Ok(SectionID {
        start: start,
        end: end,
    })
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
