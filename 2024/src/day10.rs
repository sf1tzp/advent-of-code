use std::collections::{HashMap, HashSet, VecDeque};

use crate::grid::*;

struct TopographicMap {
    grid: Grid<u8>,
    starting_points: Vec<Location>,
    peaks: Vec<Location>,
}

impl TopographicMap {
    // bfs check for valid path between two points
    fn valid_trail_exists(&self, start: Location, end: Location) -> bool {
        let mut visited = HashSet::<Location>::from([start]);
        let mut queue = VecDeque::<Location>::from([start]);

        while !queue.is_empty() {
            if let Some(current_location) = queue.pop_front() {
                if current_location == end {
                    return true;
                }

                if let Some(current_height) = self.grid.grid.get(&current_location) {
                    for d in [
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ] {
                        let next_location = get_next_point(&d, current_location);

                        if visited.contains(&next_location) {
                            continue;
                        }

                        match self.grid.grid.get(&next_location) {
                            None => {}
                            Some(next_height) => {
                                // In this scenario, only points where hight increases by 1 are valid
                                if *next_height == current_height + 1 {
                                    visited.insert(next_location);
                                    queue.push_back(next_location);
                                }
                            }
                        }
                    }
                }
            }
        }

        false
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> TopographicMap {
    let mut map = TopographicMap {
        grid: Grid::<u8>::new(input),
        starting_points: vec![],
        peaks: vec![],
    };
    for (k, v) in map.grid.grid.iter() {
        match *v {
            0 => map.starting_points.push(*k),
            9 => map.peaks.push(*k),
            _ => {}
        }
    }
    map
}

#[aoc(day10, part1)]
fn solve_part1(input: &TopographicMap) -> usize {
    // Loop all starting points
    // Loop all peaks
    // If valid path
    // Scores (starting point) += 1
    // println!("{:?}", input.grid.grid);
    // println!("possible starts: {:?}", input.starting_points);
    // println!("peaks: {:?}", input.peaks);
    let mut scores = HashMap::<Location, usize>::new();
    for s in &input.starting_points {
        for p in &input.peaks {
            // println!("finding trail between {s} and {p}");
            if input.valid_trail_exists(*s, *p) {
                scores.entry(*s).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }
    // println!("scores {:?}", scores);

    scores.values().sum()
}
