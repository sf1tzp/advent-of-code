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
fn solve_part1(input: &TreeFarm) -> usize {
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

    // println!("{:?}", input);
    input.count_visible_trees()
}

#[aoc(day8, part2, four_iterators_per_loc)]
fn solve_part2(input: &TreeFarm) -> usize {
    // println!("{}", input);
    let mut high_score = 0;
    for (location, _) in input.plots.iter() {
        let score = input.base_visibility_score(location);
        if high_score < score {
            high_score = score;
        }
    }

    high_score
}

#[aoc(day8, part2, two_iterators_per_loc)]
// This is actually 10x slower 😅 🤔
fn solve_part2_2(input: &TreeFarm) -> usize {
    // println!("{}", input);
    let mut high_score = 0;
    for (location, _) in input.plots.iter() {
        let score = input.base_visibility_score_2(location);
        if high_score < score {
            high_score = score;
        }
    }

    high_score
}

#[allow(unused_must_use)]
impl fmt::Display for TreeFarm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..=self.size.rows {
            for column_index in 0..=self.size.columns {
                let location = (row_index, column_index);
                write!(f, "{}", self.plots[&location].height);
            }
            writeln!(f);
        }
        Ok(())
    }
}

#[allow(unused_must_use)]
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
            writeln!(f);
        }
        Ok(())
    }
}

impl TreeFarm {
    pub fn count_visible_trees(&self) -> usize {
        let mut count = 0;
        for (_, tree) in self.plots.iter() {
            if tree.visible_from_outside {
                count += 1;
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

            // In this scenario 9 is the highest possible tree, so we can short circuit here
            if highest_seen == 9 {
                break;
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

            // In this scenario 9 is the highest possible tree, so we can short circuit here
            if highest_seen == 9 {
                break;
            }
        }
    }

    pub fn base_visibility_score(&self, location: &(usize, usize)) -> usize {
        let base_row = location.0;
        let base_column = location.1;
        let base_height = self.plots[location].height;
        let mut score = 1; // Start at 1, we will *= this in the loops

        // If the base location is on the edge of the forest, the score is 0
        if base_row == 0
            || base_row == self.size.rows
            || base_column == 0
            || base_column == self.size.columns
        {
            return 0;
        }

        // Iterate from the spaces adjacent to the base to the edge

        // Score Up (Base.Row -1 -> Row 0)
        let range = (0..=(base_row - 1)).rev();
        for (i, row_index) in range.enumerate() {
            if self.plots[&(row_index, base_column)].height >= base_height || row_index == 0 {
                score *= i + 1;
                break;
            }
        }

        // Score Down (Base.Row +1 -> Max Row)
        let range = (base_row + 1)..=self.size.rows;
        for (i, row_index) in range.enumerate() {
            if self.plots[&(row_index, base_column)].height >= base_height
                || row_index == self.size.rows
            {
                score *= i + 1;
                break;
            }
        }

        // Score Left (Base -> Row 0)
        let range = (0..=(base_column - 1)).rev();
        for (i, column_index) in range.enumerate() {
            if self.plots[&(base_row, column_index)].height >= base_height || column_index == 0 {
                score *= i + 1;
                break;
            }
        }

        // Score Right (Base -> Max Column)
        let range = (base_column + 1)..=self.size.columns;
        for (i, column_index) in range.enumerate() {
            if self.plots[&(base_row, column_index)].height >= base_height
                || column_index == self.size.columns
            {
                score *= i + 1;
                break;
            }
        }

        // println!("location {:?} score {}", location, score);
        score
    }

    #[allow(clippy::comparison_chain)]
    pub fn base_visibility_score_2(&self, location: &(usize, usize)) -> usize {
        let base_row = location.0;
        let base_column = location.1;
        let base_height = self.plots[location].height;
        let mut score = 1; // Start at 1, we will *= this in the loops

        // If the base location is on the edge of the forest, the score is 0
        if base_row == 0
            || base_row == self.size.rows
            || base_column == 0
            || base_column == self.size.columns
        {
            return 0;
        }

        // Score Vertical
        let mut visibility_distance = base_row;
        let range = 0..=self.size.rows;
        for row_index in range {
            // println!("row index {row_index}");
            if row_index < base_row {
                if self.plots[&(row_index, base_column)].height < base_height {
                    continue;
                }
                visibility_distance = base_row - row_index;
                // println!("Found blocker at {row_index}. New distance {visibility_distance}");
            } else if row_index == base_row {
                // println!("reached base, score {} *= {}", score, visibility_distance);
                // Reset tracker
                score *= visibility_distance;
                visibility_distance = self.size.rows - base_row;
                // println!("reset distance {visibility_distance}");
            } else if row_index > base_row && row_index < self.size.rows {
                if self.plots[&(row_index, base_column)].height < base_height {
                    continue;
                }
                visibility_distance = row_index - base_row;
                // println!("Found blocker at {row_index}, {score} *= {visibility_distance}");
                score *= visibility_distance;
                break;
            } else if row_index == self.size.rows {
                // println!("reached limit, {score} *= {visibility_distance}");
                score *= visibility_distance;
            } else {
                continue;
            }
        }

        // Score Horizontal
        let mut visibility_distance = base_column;
        let range = 0..=self.size.columns;
        for column_index in range {
            // println!("column index {column_index}");
            if column_index < base_column {
                if self.plots[&(base_row, column_index)].height < base_height {
                    continue;
                }
                visibility_distance = base_column - column_index;
                // println!("Found blocker at {column_index}. New distance {visibility_distance}");
            } else if column_index == base_column {
                // println!("reached base, score {} *= {}", score, visibility_distance);
                // Reset tracker
                score *= visibility_distance;
                visibility_distance = self.size.columns - base_column;
                // println!("reset distance {visibility_distance}");
            } else if column_index > base_column && column_index < self.size.columns {
                if self.plots[&(base_row, column_index)].height < base_height {
                    continue;
                }
                visibility_distance = column_index - base_column;
                // println!("Found blocker at {column_index}, {score} *= {visibility_distance}");
                score *= visibility_distance;
                break;
            } else if column_index == self.size.columns {
                // println!("reached limit, {score} *= {visibility_distance}");
                score *= visibility_distance;
            } else {
                continue;
            }
        }

        score
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
