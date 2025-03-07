use std::collections::{HashMap, HashSet, VecDeque};

use crate::grid::{get_next_point, Grid, Location, CARDINAL_DIRECTIONS};

struct Farm {
    grid: Grid<char>,
    regions: HashMap<usize, PlantRegion>,
}

#[derive(Default)]
struct PlantRegion {
    plant: char,
    cells: HashSet<Location>,
    boundary: HashSet<Location>,
    area: usize,
    perimeter: usize,
}

impl PlantRegion {
    fn new(plant: char) -> PlantRegion {
        let mut pr = PlantRegion::default();
        pr.plant = plant;
        pr
    }
}

impl Farm {
    fn identify_regions(&mut self) {
        let mut visisted = HashSet::<Location>::new();
        let mut region_id = 0;

        for (loc, _) in self.grid.grid.iter() {
            if !visisted.contains(loc) {
                let (region, new_visited) = self.find_region(*loc);
                self.regions.insert(region_id, region);
                visisted.extend(new_visited);
                region_id += 1;
            }
        }
    }

    // search outward from the given location
    // neighboring nodes that match the character at the target location are queued up
    // when a non-matching character, or the edge of the grid is detected,
    // the current location is added to the regions 'perimeter'.
    // locations visited in the search are returned to avoid searching the same region twice
    fn find_region(&self, loc: Location) -> (PlantRegion, HashSet<Location>) {
        let plant = match self.grid.get(loc) {
            Some(c) => c,
            None => panic!("tried to find a region that wasn't on the grid: {}", loc),
        };
        let mut region = PlantRegion::new(plant);
        let mut queue = VecDeque::<Location>::from([loc]);
        let mut visited = HashSet::<Location>::from([loc]);

        while !queue.is_empty() {
            let current = queue.pop_front().unwrap();
            region.cells.insert(current);
            for dir in CARDINAL_DIRECTIONS {
                let mut is_boundary = false;
                let next = get_next_point(&dir, current);
                match self.grid.get(next) {
                    Some(c) => {
                        if c == plant && !visited.contains(&next) {
                            queue.push_back(next);
                            visited.insert(next);
                        } else if c != plant {
                            is_boundary = true;
                            region.perimeter += 1;
                        }
                    }
                    None => {
                        is_boundary = true;
                        region.perimeter += 1;
                    }
                }

                if is_boundary {
                    region.boundary.insert(current);
                }
            }
        }

        // Note: the region perimeter was counted separately than just `region.boundary.len()`, because
        // corner locations will technically add 2 to the perimeter

        region.area = region.cells.len();

        (region, visited)
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> Farm {
    let mut farm = Farm {
        grid: Grid::<char>::new(input),
        regions: HashMap::new(),
    };
    farm.identify_regions();
    farm
}

#[aoc(day12, part1)]
fn solve_part1(input: &Farm) -> usize {
    input
        .regions
        .iter()
        .map(|(_, r)| r.area * r.perimeter)
        .sum()
}
