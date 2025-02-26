use std::{collections::HashSet, fmt};

use crate::grid::*;

#[derive(Clone)]
struct Guard {
    location: (usize, usize),
    direction: Direction,
}

#[derive(Clone)]
struct Room {
    grid: Grid<char>,
    guard: Guard,
}

impl Room {
    fn get_next_guard_location(&mut self) -> Option<(usize, usize)> {
        let next_point = get_next_point(&self.guard.direction, self.guard.location);
        // Check to see if the next point is on the grid
        match self.grid.grid.get(&next_point) {
            // if it is, see if it is an unnocupied space '.' or if the guard needs to turn '#'
            Some(c) => match c {
                '.' => return Some(next_point),
                '#' => {
                    match self.guard.direction {
                        Direction::Up => self.guard.direction = Direction::Right,
                        Direction::Right => self.guard.direction = Direction::Down,
                        Direction::Down => self.guard.direction = Direction::Left,
                        Direction::Left => self.guard.direction = Direction::Up,
                        _ => panic!(
                            "The guard got turned around {:?} at {:?}",
                            self.guard.direction, self.guard.location
                        ),
                    };
                    // If the guard turned, they do not move this turn
                    return Some(self.guard.location);
                }
                _ => panic!("unknown character {c} found at {:?}", next_point),
            },
            // If the next point is off the grid, return None to signal their exit
            None => return None,
        };
    }
}

impl fmt::Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..self.grid.size.rows {
            for column_index in 0..self.grid.size.columns {
                let location = (row_index, column_index);
                if location == self.guard.location {
                    write!(f, "^")?;
                } else {
                    let c = self.grid.grid.get(&location).unwrap();
                    write!(f, "{}", c)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[aoc_generator(day6)]
fn parse_input(input: &str) -> Room {
    let mut room = Room {
        grid: Grid::<char>::new(input),
        guard: Guard {
            location: (0, 0),
            direction: Direction::Up,
        },
    };

    // Set the guard's position (todo: maybe do above?)
    match room.grid.find('^') {
        Some((x, y)) => {
            room.guard.location = (x, y);
            room.grid.grid.insert((x, y), '.');
        }
        None => panic!("no guard location found!"),
    };

    room
}

#[aoc(day6, part1)]
fn solve_part_1(input: &Room) -> usize {
    let mut room: Room = input.clone();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    // Loop until the guard exits the grid, recording visited spaces
    while let Some(next_point) = room.get_next_guard_location() {
        // println!("{}", room);
        // println!(
        //     "Guard at {:?}, fn returned {:?}",
        //     room.guard.location, location
        // );

        // If the guard will move this turn, record their current position before updating
        if next_point != room.guard.location {
            visited.insert(room.guard.location);
            room.guard.location = next_point;
            // println!("The guard moved. Count: {}", visited.len());
        }
        // println!("==========");
    }

    // When the guard would have exited the grid, record their last position for an accurate count
    visited.insert(room.guard.location);
    // println!("{:?}", visited);
    visited.len()
}

#[cfg(test)]
mod tests {
    #[test]
    fn tuple_equality() {
        assert_ne!((1, 1), (2, 2))
    }
}
