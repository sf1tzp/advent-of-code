use crate::grid::{get_next_point, Direction, Grid};

use anyhow::{anyhow, Result};

type WordSearch = Grid<char>;

impl WordSearch {
    fn search(&self, starting_point: (usize, usize), word: &str) -> Result<usize> {
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

            current_point = get_next_point(&direction, current_point);
        }

        true
    }

    fn find_cross(&self, starting_point: (usize, usize), word: &str) -> Result<bool> {
        let mut total = 0;
        let word: Vec<char> = word.chars().collect();

        // to form an X the word must be odd in length
        assert!(word.len() % 2 != 0);
        // find the character in the middle
        let pivot = word[word.len() / 2];

        match self.grid.get(&starting_point) {
            None => return Err(anyhow!("invalid starting location")),
            Some(c) => {
                if c != &pivot {
                    return Ok(false);
                }
            }
        }

        // move to top right and search down-left
        let p = get_next_point(&Direction::UpRight, starting_point);
        if self.search_direction(Direction::DownLeft, p, &word) {
            total += 1;
        }
        // move to top left and search down-right
        let p = get_next_point(&Direction::UpLeft, starting_point);
        if self.search_direction(Direction::DownRight, p, &word) {
            total += 1;
        }
        // move to bottom right and search up-left
        let p = get_next_point(&Direction::DownRight, starting_point);
        if self.search_direction(Direction::UpLeft, p, &word) {
            total += 1;
        }
        // move to bottom left and search up-right
        let p = get_next_point(&Direction::DownLeft, starting_point);
        if self.search_direction(Direction::UpRight, p, &word) {
            total += 1;
        }

        Ok(total == 2)
    }
}

#[aoc_generator(day4)]
fn parse_input(input: &str) -> WordSearch {
    let wordsearch = WordSearch::new(input);
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
            match input.search(location, word) {
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

#[aoc(day4, part2)]
fn solve_part2(input: &WordSearch) -> usize {
    let mut total = 0;
    let word = "MAS";
    for row_index in 0..input.size.rows {
        for colum_index in 0..input.size.columns {
            let location = (row_index, colum_index);

            // println!("Searching from location {:?}", location);
            match input.find_cross(location, word) {
                Ok(found) => {
                    if found {
                        total += 1;
                    }
                }
                Err(e) => panic!("WTF {e}"),
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_int_division() {
        assert_eq!(3 / 2, 1);
        assert_eq!(5 / 2, 2);
        let word = vec!['M', 'A', 'S'];
        let pivot = word[word.len() / 2];
        assert_eq!(pivot, 'A');
    }
}
