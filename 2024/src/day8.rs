use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::grid::*;

struct Map {
    grid: Grid<char>,
    antennas: HashMap<(usize, usize), char>,
    frequencies: HashSet<char>,
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
}

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
        frequencies: HashSet::new(),
    };

    for (k, v) in map.grid.grid.iter() {
        if *v != '.' {
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_antinodes() {
        // simple diagonal
        let antinode = find_antinode_location((0, 0), (5, 5));
        assert_eq!(antinode, Some((10, 10)));
        // 2:1 slope
        let antinode = find_antinode_location((0, 0), (1, 2));
        assert_eq!(antinode, Some((2, 4)));
        // location is negative
        let antinodes = find_antinode_location((5, 5), (0, 0));
        assert_eq!(antinodes, None);
    }
}
