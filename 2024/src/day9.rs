use std::fmt;
use std::vec;

use crate::ASCII_DIGITS;

#[derive(Clone)]
struct Disk {
    disk_map: Vec<usize>,
    contents: Vec<Option<usize>>,
}

impl Disk {
    fn generate_contents_from_disk_map(&mut self) {
        let contents: Vec<Option<usize>> = self
            .disk_map
            .iter()
            .enumerate()
            .map(|(i, n)| match i % 2 != 0 {
                // Create a vector the length of the size of the block filled with None for empty blocks, or the Id of the resulting file
                true => vec![None; *n],
                false => {
                    let block_id = match i == 0 {
                        true => i,
                        false => i / 2,
                    };
                    vec![Some(block_id); *n]
                }
            })
            .flatten()
            .collect();
        self.contents = contents
    }

    fn compact(&mut self) {
        // For each block
        // if block is empty (i)
        // find right-most non empty block (j)
        // swap (i,j)
        let mut i = 0;
        let mut j = self.get_last_filled_index();
        while i < j {
            match self.contents[i] {
                None => {
                    self.contents[i] = self.contents[j];
                    self.contents[j] = None;
                    j = self.get_last_filled_index();
                    // println!("{self}")
                }
                _ => {}
            }
            i += 1;
        }
    }

    fn get_last_filled_index(&self) -> usize {
        let mut j = self.contents.len() - 1;
        while j > 0 {
            match self.contents[j] {
                None => j -= 1,
                _ => return j,
            }
        }
        j
    }

    fn checksum(&self) -> usize {
        self.contents
            .iter()
            .enumerate()
            .map(|(i, c)| match c {
                None => 0,
                _ => {
                    let n = c.unwrap();
                    i * n as usize
                }
            })
            .sum()
    }
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.contents.len() {
            let _ = match self.contents[i] {
                Some(id) => write!(f, "({id})"),
                None => write!(f, "."),
            };
        }
        writeln!(f, "")
    }
}

#[aoc_generator(day9)]
fn parse_input(input: &str) -> Disk {
    let mut d = Disk {
        disk_map: input
            .chars()
            .filter(|c| ASCII_DIGITS.contains(c))
            .map(|n| n.to_digit(10).unwrap() as usize) // parse().unwrap())
            .collect(),
        contents: vec![],
    };
    d.generate_contents_from_disk_map();
    d
}

#[aoc(day9, part1)]
fn solve_part1(input: &Disk) -> usize {
    // println!("{:?}", input.disk_map);
    println!("{}", input);
    // println!("{:?}", input.contents);
    // println!("----");
    let mut map = input.clone();
    map.compact();
    // println!("{}", map);
    map.checksum()
}
