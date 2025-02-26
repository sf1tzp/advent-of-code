use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GridSize {
    pub rows: usize,
    pub columns: usize,
}

#[derive(Clone)]
pub struct Grid<T> {
    pub size: GridSize,
    pub grid: HashMap<(usize, usize), T>,
}

#[derive(Debug, Clone)]
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

pub fn get_next_point(direction: &Direction, current_point: (usize, usize)) -> (usize, usize) {
    match direction {
        Direction::Up => return (current_point.0 - 1, current_point.1),
        Direction::UpRight => return (current_point.0 - 1, current_point.1 + 1),
        Direction::Right => return (current_point.0, current_point.1 + 1),
        Direction::DownRight => return (current_point.0 + 1, current_point.1 + 1),
        Direction::Down => return (current_point.0 + 1, current_point.1),
        Direction::DownLeft => return (current_point.0 + 1, current_point.1 - 1),
        Direction::Left => return (current_point.0, current_point.1 - 1),
        Direction::UpLeft => return (current_point.0 - 1, current_point.1 - 1),
    }
}

impl Grid<char> {
    pub fn new(input: &str) -> Grid<char> {
        let mut g = Grid::<char> {
            grid: HashMap::<(usize, usize), char>::new(),
            size: GridSize {
                rows: input.lines().count(),
                columns: input.lines().nth(0).unwrap().len(),
            },
        };

        for (row_index, line) in input.lines().enumerate() {
            for (column_index, letter) in line.chars().enumerate() {
                let location = (row_index, column_index);
                g.grid.insert(location, letter);
            }
        }

        g
    }

    pub fn find(&self, target: char) -> Option<(usize, usize)> {
        for row_index in 0..self.size.rows {
            for column_index in 0..self.size.columns {
                if let Some(c) = self.grid.get(&(row_index, column_index)) {
                    if target == *c {
                        return Some((row_index, column_index));
                    }
                }
            }
        }
        None
    }
}
