use std::{collections::HashMap, fmt};

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Location {
    pub row: usize,
    pub column: usize,
}

impl Location {
    pub fn new(row: usize, column: usize) -> Location {
        Location { row, column }
    }
}

impl fmt::Display for Location {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.row, self.column)
    }
}

#[derive(Debug, Clone)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Clone)]
pub struct Grid<T> {
    pub size: GridSize,
    pub grid: HashMap<Location, T>,
}

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub static CARDINAL_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub fn get_next_point(direction: &Direction, current_point: Location) -> Location {
    match direction {
        Direction::Up => return Location::new(current_point.row - 1, current_point.column),
        Direction::UpRight => {
            return Location::new(current_point.row - 1, current_point.column + 1)
        }
        Direction::Right => return Location::new(current_point.row, current_point.column + 1),
        Direction::DownRight => {
            return Location::new(current_point.row + 1, current_point.column + 1)
        }
        Direction::Down => return Location::new(current_point.row + 1, current_point.column),
        Direction::DownLeft => {
            return Location::new(current_point.row + 1, current_point.column - 1)
        }
        Direction::Left => return Location::new(current_point.row, current_point.column - 1),
        Direction::UpLeft => return Location::new(current_point.row - 1, current_point.column - 1),
    }
}

impl Grid<char> {
    pub fn new(input: &str) -> Grid<char> {
        let mut g = Grid::<char> {
            grid: HashMap::<Location, char>::new(),
            size: GridSize {
                rows: input.lines().count(),
                columns: input.lines().nth(0).unwrap().len(),
            },
        };

        for (row_index, line) in input.lines().enumerate() {
            for (column_index, letter) in line.chars().enumerate() {
                let location = Location {
                    row: row_index,
                    column: column_index,
                };
                g.grid.insert(location, letter);
            }
        }

        g
    }

    pub fn find(&self, target: char) -> Option<(usize, usize)> {
        for row_index in 0..self.size.rows {
            for column_index in 0..self.size.columns {
                let location = Location::new(row_index, column_index);
                if let Some(c) = self.grid.get(&location) {
                    if target == *c {
                        return Some((row_index, column_index));
                    }
                }
            }
        }
        None
    }
}

impl Grid<u8> {
    pub fn new(input: &str) -> Grid<u8> {
        let mut g = Grid::<u8> {
            grid: HashMap::<Location, u8>::new(),
            size: GridSize {
                rows: input.lines().count(),
                columns: input.lines().nth(0).unwrap().len(),
            },
        };

        for (row_index, line) in input.lines().enumerate() {
            for (column_index, letter) in line.chars().enumerate() {
                let location = Location::new(row_index, column_index);
                g.grid.insert(location, letter.to_digit(10).unwrap() as u8);
            }
        }

        g
    }
}
