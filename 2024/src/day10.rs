use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

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
                    for d in CARDINAL_DIRECTIONS {
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

    // dfs with parent pointer map for path reconstruction
    fn count_of_trails(&self, start: Location, end: Location) -> usize {
        let mut trails = Vec::new();
        let mut parent_map: HashMap<Location, Location> = HashMap::new();
        let mut global_visited: HashSet<Location> = HashSet::from([start]);

        struct SearchState {
            current_point: Location,
            direction_index: usize,
        }

        let mut search_stack: VecDeque<SearchState> = VecDeque::new();
        search_stack.push_back(SearchState {
            current_point: start,
            direction_index: 0,
        });

        while !search_stack.is_empty() {
            let s = search_stack.pop_front().unwrap();

            if s.direction_index >= CARDINAL_DIRECTIONS.len() {
                // We're done with this point, remove from visited for backtracking
                global_visited.remove(&s.current_point);
                continue;
            }

            // Queue up next direction to try
            search_stack.push_front(SearchState {
                current_point: s.current_point,
                direction_index: s.direction_index + 1,
            });

            let next_point =
                get_next_point(&CARDINAL_DIRECTIONS[s.direction_index], s.current_point);

            if global_visited.contains(&next_point) {
                continue;
            }

            if let Some(next_height) = self.grid.grid.get(&next_point) {
                let current_height = self.grid.grid.get(&s.current_point).unwrap();
                if *next_height == current_height + 1 {
                    parent_map.insert(next_point, s.current_point);
                    global_visited.insert(next_point);

                    if next_point == end {
                        // Reconstruct path for the solution
                        let mut path = vec![next_point];
                        let mut current = next_point;
                        while current != start {
                            current = parent_map[&current];
                            path.push(current);
                        }
                        path.reverse();
                        trails.push(path);

                        // Remove this point from visited to allow other paths
                        global_visited.remove(&next_point);
                        parent_map.remove(&next_point);
                        continue;
                    }

                    // Queue next point
                    search_stack.push_front(SearchState {
                        current_point: next_point,
                        direction_index: 0,
                    });
                }
            }
        }

        trails.len()
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

#[aoc(day10, part2)]
fn solve_part2(input: &TopographicMap) -> usize {
    let mut scores = HashMap::<Location, usize>::new();
    for s in &input.starting_points {
        for p in &input.peaks {
            let score = input.count_of_trails(*s, *p);
            scores
                .entry(*s)
                .and_modify(|x| *x += score)
                .or_insert(score);
        }
    }

    scores.values().sum()
}
