use std::{
    collections::{HashMap, HashSet},
    fmt,
};

#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, Clone)]
struct TopographicMap {
    map: HashMap<Point, usize>,
    width: usize,
    height: usize,
    start: Point,
    end: Point,
}

impl TopographicMap {
    fn new_from_input(input: &str) -> Self {
        let map: HashMap<Point, usize> = input
            .lines()
            .enumerate()
            .flat_map(|(i, line)| {
                line.chars()
                    .enumerate()
                    .map(|(j, c)| (Point { x: j, y: i }, c as usize))
                    .collect::<Vec<(Point, usize)>>()
            })
            .collect::<HashMap<Point, usize>>();

        let width = input.lines().next().unwrap().len();
        let height = input.lines().count();
        let mut topographic_map = TopographicMap {
            map,
            width,
            height,
            start: Point { x: 0, y: 0 },
            end: Point { x: 0, y: 0 },
        };
        topographic_map.locate_start_and_end();
        topographic_map
    }

    #[allow(non_snake_case)]
    fn locate_start_and_end(&mut self) {
        let S_ASCII: usize = 83;
        let E_ASCII: usize = 69;
        let a_ASCII: usize = 97;
        let z_ASCII: usize = 122;

        for i in 0..self.height {
            for j in 0..self.width {
                let point = Point { x: j, y: i };
                let height = self.map.get(&point).unwrap();
                if *height == S_ASCII {
                    self.start = point;
                } else if *height == E_ASCII {
                    self.end = point;
                }
            }
        }
        self.map
            .entry(self.start)
            .and_modify(|height| *height = a_ASCII);
        self.map
            .entry(self.end)
            .and_modify(|height| *height = z_ASCII);
    }

    fn shortest_path_a_star(&self) -> Vec<Point> {
        let start = self.start;
        let end = self.end;

        let mut distance_map: HashMap<Point, usize> = self
            .map
            .keys()
            .map(|p| (Point { x: p.x, y: p.y }, usize::MAX))
            .collect::<HashMap<Point, usize>>();

        let mut unvisited: HashSet<Point> = self
            .map
            .keys()
            .map(|p| (Point { x: p.x, y: p.y }))
            .collect::<HashSet<Point>>();

        let mut previous_point = HashMap::<Point, Point>::new();

        distance_map.entry(start).and_modify(|d| *d = 0);

        while !unvisited.is_empty() {
            let (current, _) = distance_map // speed this up
                .iter()
                .filter(|(p, _)| unvisited.contains(p))
                .min_by_key(|(_, d)| *d)
                .unwrap();

            let current = *current; // copy the borrowed value

            if current == end {
                break;
            }
            unvisited.remove(&current);

            let neighbors = self.get_neighbors(&current);
            for neighbor in neighbors {
                if !unvisited.contains(&neighbor) {
                    continue;
                }

                if !self.is_point_selectable(&current, &neighbor) {
                    continue;
                }

                let new_distance = distance_map[&current] + 1;
                let neighbor_distance = distance_map[&neighbor];

                if new_distance < neighbor_distance {
                    distance_map
                        .entry(neighbor)
                        .and_modify(|s| *s = new_distance)
                        .or_insert(new_distance);

                    previous_point
                        .entry(neighbor)
                        .and_modify(|p| *p = current)
                        .or_insert(current);
                }
            }
        }

        // Traverse previous backwards to get the path
        let mut path = vec![];
        let mut current = end;
        while current != start {
            path.push(current);
            current = previous_point[&current];
        }
        path.push(start);
        path.iter().rev().copied().collect()
    }

    // only get the adjacent points, no diagonals allowed.
    fn get_neighbors(&self, point: &Point) -> Vec<Point> {
        let mut neighbors = Vec::with_capacity(4);
        if point.x > 0 {
            neighbors.push(Point {
                x: point.x - 1,
                y: point.y,
            });
        }
        if point.x < self.width - 1 {
            neighbors.push(Point {
                x: point.x + 1,
                y: point.y,
            });
        }
        if point.y > 0 {
            neighbors.push(Point {
                x: point.x,
                y: point.y - 1,
            });
        }
        if point.y < self.height - 1 {
            neighbors.push(Point {
                x: point.x,
                y: point.y + 1,
            });
        }
        neighbors
    }

    fn is_point_selectable(&self, current: &Point, candidate: &Point) -> bool {
        let current_height = self.map[current];
        let candidate_height = self.map[candidate];

        if candidate_height > current_height && candidate_height - current_height > 1 {
            return false;
        }

        true
    }

    #[allow(dead_code)]
    fn print_path(&self, path: &[Point]) {
        enum Direction {
            Up,
            Down,
            Left,
            Right,
        }

        fn get_direction(current: &Point, next: &Point) -> Direction {
            if current.x == next.x {
                if current.y > next.y {
                    return Direction::Up;
                } else {
                    return Direction::Down;
                }
            }
            if current.x > next.x {
                return Direction::Left;
            }
            Direction::Right
        }

        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point { x: col, y: row };
                if path.contains(&point) {
                    let index = path.iter().position(|p| *p == point).unwrap();
                    let current = path[index];
                    if current == self.start {
                        print!("S");
                        continue;
                    }
                    if current == self.end {
                        print!("E");
                        continue;
                    }

                    let next = path[index + 1];
                    let c = match get_direction(&current, &next) {
                        Direction::Up => '^',
                        Direction::Down => 'v',
                        Direction::Left => '<',
                        Direction::Right => '>',
                    };
                    print!("{}", c);
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
}

// implement Display for TopographicMap. Print out each character in the grid:
impl fmt::Display for TopographicMap {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let start = self.start;
        let end = self.end;

        for row in 0..self.height {
            for col in 0..self.width {
                let point = Point { x: col, y: row };
                let c = match point {
                    point if point == start => 'S',
                    point if point == end => 'E',
                    _ => self.map[&point] as u8 as char,
                };
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }

        writeln!(f, "Start: {:?}", self.start)?;
        writeln!(f, "End: {:?}", self.end)?;
        Ok(())
    }
}

#[aoc_generator(day12)]
fn parse_input(input: &str) -> TopographicMap {
    TopographicMap::new_from_input(input)
}

#[aoc(day12, part1)]
fn solve_part1(map: &TopographicMap) -> usize {
    let path = map.shortest_path_a_star();

    // print!("{}", map);
    // map.print_path(&path);

    path.len() - 1
}
