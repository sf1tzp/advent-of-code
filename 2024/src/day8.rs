use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use itertools::Itertools;

use crate::{grid::*, ASCII_DIGITS, ASCII_LOWERCASE, ASCII_UPPERCASE};

#[derive(Clone)]
struct Map {
    grid: Grid<char>,
    antennas: HashMap<(usize, usize), char>,
    antinodes: HashSet<(usize, usize)>,
    frequencies: HashSet<char>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row_index in 0..self.grid.size.rows {
            for column_index in 0..self.grid.size.columns {
                let location = (row_index, column_index);
                if let Some(_) = self.antinodes.get(&location) {
                    write!(f, "#")?;
                } else if let Some(a) = self.antennas.get(&location) {
                    write!(f, "{a}")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn find_matching_antenna(&self, target: char) -> Vec<(usize, usize)> {
        self.grid
            .grid
            .keys()
            .filter(|location| self.grid.grid.get(location).map_or(false, |a| *a == target))
            .cloned()
            .collect()
    }

    // Note: Part 2 implementation
    fn find_antinodes(&mut self, frequency: char) -> Vec<(usize, usize)> {
        let antennas = self.find_matching_antenna(frequency);
        let mut antinodes: Vec<(usize, usize)> = vec![];
        // for each antenna
        for i in 0..antennas.len() {
            let a = antennas[i];
            // compare with each other antenna
            for j in 0..antennas.len() {
                if i == j {
                    continue;
                }
                let b = antennas[j];

                // get antinode locations
                antinodes.extend(self.find_antinode_locations(a, b));
                antinodes.extend(self.find_antinode_locations(b, a));
            }
        }

        antinodes
    }

    // Note: Part 2 implementation
    // antinodes appear at points along the line formed by two antenna
    fn find_antinode_locations(
        &mut self,
        first: (usize, usize),
        second: (usize, usize),
    ) -> Vec<(usize, usize)> {
        let mut antinodes = Vec::new();
        if first == second {
            return antinodes;
        }
        // determine the "slope" betwen points
        let dx = second.0 as isize - first.0 as isize;
        let dy = second.1 as isize - first.1 as isize;

        // calculate the distance to the next coordinate
        let step_size = get_step_size(dx.abs() as usize, dy.abs() as usize);
        let step_x = dx / step_size as isize;
        let step_y = dy / step_size as isize;

        let mut x = second.0 as isize;
        let mut y = second.1 as isize;

        // println!("starting at {x},{y}, stepping {step_x},{step_y}");

        // step along the line until reaching the edge of the grid
        while x >= 0
            && y >= 0
            && (x as usize) < self.grid.size.rows
            && (y as usize) < self.grid.size.columns
        {
            // if let Some(a) = self.antennas.get(&(x as usize, y as usize)) {
            //     println!("Found antinode at antenna {a} location {x},{y}");
            // }

            antinodes.push((x as usize, y as usize));
            x += step_x;
            y += step_y;
        }

        for a in antinodes.iter() {
            self.antinodes.insert((a.0, a.1));
        }

        antinodes
    }
}

fn get_step_size(mut a: usize, mut b: usize) -> usize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }

    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp
    }
    a
}

// Note: Part 1 implementation
// antinodes appear at points along the line formed by two antenna,
// twice the distance from the first antenna as the second antenna is
fn find_antinode_location(first: (usize, usize), second: (usize, usize)) -> Option<(usize, usize)> {
    if first == second {
        return None;
    }
    // determine the "slope" betwen points
    let dx = second.0 as isize - first.0 as isize;
    let dy = second.1 as isize - first.1 as isize;

    // calculate the antinode coordinate along the line
    let ax = second.0 as isize + dx;
    let ay = second.1 as isize + dy;

    // if the point exists in negative space, reject it
    if ax < 0 || ay < 0 {
        return None;
    }

    // convert back to usize and return
    Some((ax as usize, ay as usize))
}

// Note: Part 1 implementation
fn find_antinodes(antenna: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut antinodes: Vec<Option<(usize, usize)>> = vec![];
    // for each antenna
    for i in 0..antenna.len() {
        let a = antenna[i];
        // compare with each other antenna
        for j in 0..antenna.len() {
            if i == j {
                continue;
            }
            let b = antenna[j];

            // get antinode locations
            antinodes.push(find_antinode_location(a, b));
            antinodes.push(find_antinode_location(b, a));
        }
    }

    antinodes.iter().flatten().cloned().collect()
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Map {
    let mut map = Map {
        grid: Grid::<char>::new(input),
        antennas: HashMap::new(),
        antinodes: HashSet::new(),
        frequencies: HashSet::new(),
    };

    for (k, v) in map.grid.grid.iter() {
        if ASCII_DIGITS.contains(v) || ASCII_LOWERCASE.contains(v) || ASCII_UPPERCASE.contains(v) {
            map.antennas.insert(*k, *v);
            map.frequencies.insert(*v);
        }
    }

    map
}

#[aoc(day8, part1)]
fn sovle_part1(input: &Map) -> usize {
    let map = input;
    let mut antinodes: Vec<(usize, usize)> = vec![];
    for f in map.frequencies.iter() {
        // println!("finding antinodes for frequency {f}");
        let antennas = map.find_matching_antenna(*f);
        // println!("checking antinodes between {} antenna", antenna.len());
        antinodes.extend(find_antinodes(antennas));
        // println!("running total: {}", antinodes.len());
        // println!("antinodes: {:?}", antinodes);
    }

    // return a count of unique locations that are on the map grid
    antinodes
        .iter()
        .unique()
        .filter(|x| map.grid.grid.get(x).is_some())
        .count()
}

#[aoc(day8, part2)]
fn solve_part2(input: &Map) -> usize {
    let mut map = input.clone();
    let mut antinodes: Vec<(usize, usize)> = vec![];
    for f in input.frequencies.iter() {
        antinodes.extend(map.find_antinodes(*f));
    }
    println!("{}", map);
    antinodes.iter().unique().count()
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_antinodes() {
    //     // simple diagonal
    //     let antinode = find_antinode_locations((0, 0), (5, 5));
    //     assert_eq!(antinode, Some((10, 10)));
    //     // 2:1 slope
    //     let antinode = find_antinode_locations((0, 0), (1, 2));
    //     assert_eq!(antinode, Some((2, 4)));
    //     // location is negative
    //     let antinodes = find_antinode_locations((5, 5), (0, 0));
    //     assert_eq!(antinodes, None);
    // }
    // #[test]
    // fn test_get_steps() {
    //     let steps = get_steps((0, 0), (5, 5));
    //     println!("1: {:?}", steps);
    //     let steps = get_steps((5, 5), (0, 0));
    //     println!("2: {:?}", steps);
    // }
}
