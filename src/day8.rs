use std::collections::HashMap;
use std::fmt;
use std::ops::RangeInclusive;

use anyhow::anyhow;
use itertools::Either;

use crate::ASCII_DIGITS;

#[derive(Clone, Copy)]
struct Tree {
    height: usize,
    visible_from_outside: bool,
}

#[derive(Clone)]
struct Dimensions {
    rows: usize,
    columns: usize,
}

#[derive(Clone)]
struct TreeFarm {
    size: Dimensions,
    plots: HashMap<(usize, usize), Tree>,
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> TreeFarm {
    let mut farm = TreeFarm {
        plots: HashMap::new(),
        size: Dimensions {
            rows: input.lines().count() - 1,
            columns: input.lines().next().unwrap().len() - 1,
        },
    };

    for (row_index, line) in input.lines().enumerate() {
        for (column_index, tree_height) in line.chars().enumerate() {
            let location = (row_index, column_index);
            let _ = match parse_height(tree_height) {
                Ok(tree) => farm.plots.insert(location, tree),
                Err(error) => panic!(
                    "Failed to parse input: {} at {},{}",
                    error, row_index, column_index
                ),
            };
        }
    }

    farm
}

#[aoc(day8, part1)]
fn count_visible(input: &TreeFarm) -> usize {
    let mut input: TreeFarm = input.clone();
    // println!("{}", input);
    for row_index in 0..=input.size.rows {
        input.visible_trees_in_row(row_index, true);
        input.visible_trees_in_row(row_index, false);
    }
    for column_index in 0..input.size.columns {
        input.visible_trees_in_col(column_index, true);
        input.visible_trees_in_col(column_index, false);
    }

    let count = input.count_visible_trees();
    // println!("{:?}", input);
    count
}

impl fmt::Display for TreeFarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..=self.size.rows {
            for column_index in 0..=self.size.columns {
                let location = (row_index, column_index);
                write!(f, "{}", self.plots[&location].height);
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

// Debug displays the heights of visible trees, and '-' for obscured ones.
impl fmt::Debug for TreeFarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..=self.size.rows {
            for column_index in 0..=self.size.columns {
                let location = (row_index, column_index);
                if self.plots[&location].visible_from_outside {
                    write!(f, "{}", self.plots[&(row_index, column_index)].height);
                } else {
                    write!(f, "-");
                }
            }
            write!(f, "\n");
        }
        Ok(())
    }
}

impl TreeFarm {
    pub fn count_visible_trees(&self) -> usize {
        let mut count = 0;
        for row_index in 0..=self.size.rows {
            for column_index in 0..=self.size.columns {
                let location = (row_index, column_index);
                if self.plots[&location].visible_from_outside {
                    count += 1;
                }
            }
        }
        count
    }

    pub fn visible_trees_in_row(&mut self, row_index: usize, reversed: bool) {
        let mut highest_seen = 0;
        let range = 0..=self.size.columns;
        let counting_order = get_counting_order(range, reversed);

        for (i, column_index) in counting_order.enumerate() {
            let location = (row_index, column_index);
            let tree_height = self.plots[&location].height;
            if i == 0 || tree_height > highest_seen {
                highest_seen = tree_height;
                self.plots.get_mut(&location).unwrap().visible_from_outside = true;
            }
        }
    }

    pub fn visible_trees_in_col(&mut self, column_index: usize, reversed: bool) {
        let mut highest_seen = 0;
        let range = 0..=self.size.rows;
        let counting_order = get_counting_order(range, reversed);

        for (i, row_index) in counting_order.enumerate() {
            let location = (row_index, column_index);
            let tree_height = self.plots[&location].height;
            if i == 0 || tree_height > highest_seen {
                highest_seen = tree_height;
                self.plots.get_mut(&location).unwrap().visible_from_outside = true;
            }
        }
    }
}

fn parse_height(input: char) -> anyhow::Result<Tree> {
    if !ASCII_DIGITS.contains(&input) {
        return Err(anyhow!("{} is not a digit", input));
    }
    Ok(Tree {
        // To parse an ASCII Digit char into the correct numerical value, we have
        // to subtract the ASCII character offset ('0' as usize == 48)
        height: input as usize - '0' as usize,
        visible_from_outside: false, // default to false
    })
}

// Use the itertools Either type wrapper to work with Range and Rev<Range> interchangably
fn get_counting_order(
    range: RangeInclusive<usize>,
    reversed: bool,
) -> Either<impl Iterator<Item = usize>, impl Iterator<Item = usize>> {
    match reversed {
        true => Either::Left(range.rev()),
        false => Either::Right(range),
    }
}
