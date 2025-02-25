use std::collections::HashMap;

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Dimensions {
    rows: usize,
    columns: usize,
}

struct WordSearch {
    size: Dimensions,
    grid: HashMap<(usize, usize), char>,
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl WordSearch {
    fn find(&self, starting_point: (usize, usize), word: &str) -> Result<usize> {
        let mut total = 0;
        let word: Vec<char> = word.chars().collect();
        // Check the starting point to see if it's valid, and matches the first character of the word
        match self.grid.get(&starting_point) {
            None => {
                return Err(anyhow!(
                    "Starting point {:?} is outside of the grid dimensions {:?}",
                    starting_point,
                    self.size
                ))
            }
            Some(c) => match word.first() {
                None => return Err(anyhow!("Word is empty")),
                Some(first) => {
                    if c != first {
                        return Ok(0);
                    }
                }
            },
        };

        // search up
        if self.search_direction(Direction::Up, starting_point, &word) {
            total += 1;
        }
        // search up-right
        if self.search_direction(Direction::UpRight, starting_point, &word) {
            total += 1;
        }
        // search right
        if self.search_direction(Direction::Right, starting_point, &word) {
            total += 1;
        }
        // search down-right
        if self.search_direction(Direction::DownRight, starting_point, &word) {
            total += 1;
        }
        // search down
        if self.search_direction(Direction::Down, starting_point, &word) {
            total += 1;
        }
        // search down-left
        if self.search_direction(Direction::DownLeft, starting_point, &word) {
            total += 1;
        }
        // search left
        if self.search_direction(Direction::Left, starting_point, &word) {
            total += 1;
        }
        // search up-left
        if self.search_direction(Direction::UpLeft, starting_point, &word) {
            total += 1;
        }

        Ok(total)
    }

    fn search_direction(
        &self,
        direction: Direction,
        starting_point: (usize, usize),
        word: &Vec<char>,
    ) -> bool {
        let mut current_point = starting_point;

        for i in 0..word.len() {
            match self.grid.get(&current_point) {
                None => {
                    return false;
                }
                Some(c) => {
                    if c != &word[i] {
                        return false;
                    }
                }
            };

            match direction {
                Direction::Up => current_point = (current_point.0 - 1, current_point.1),
                Direction::UpRight => current_point = (current_point.0 - 1, current_point.1 + 1),
                Direction::Right => current_point = (current_point.0, current_point.1 + 1),
                Direction::DownRight => current_point = (current_point.0 + 1, current_point.1 + 1),
                Direction::Down => current_point = (current_point.0 + 1, current_point.1),
                Direction::DownLeft => current_point = (current_point.0 + 1, current_point.1 - 1),
                Direction::Left => current_point = (current_point.0, current_point.1 - 1),
                Direction::UpLeft => current_point = (current_point.0 - 1, current_point.1 - 1),
            }
        }

        true
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> WordSearch {
    let mut wordsearch = WordSearch {
        grid: HashMap::new(),
        size: Dimensions {
            rows: input.lines().count(),
            columns: input.lines().nth(0).unwrap().len(),
        },
    };

    for (row_index, line) in input.lines().enumerate() {
        for (column_index, letter) in line.chars().enumerate() {
            let location = (row_index, column_index);
            wordsearch.grid.insert(location, letter);
        }
    }
    wordsearch
}

#[aoc(day4, part1)]
fn solve_part1(input: &WordSearch) -> usize {
    let mut total = 0;
    let word = "XMAS";
    for row_index in 0..input.size.rows {
        for colum_index in 0..input.size.columns {
            let location = (row_index, colum_index);
            // println!("Searching from location {:?}", location);
            match input.find(location, word) {
                Ok(n) => {
                    // println!("Found {n} matches there");
                    total += n;
                }
                Err(e) => panic!("WTF {e}"),
            }
        }
    }
    total
}
