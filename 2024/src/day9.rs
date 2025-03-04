use std::collections::BTreeMap;
use std::fmt;
use std::vec;

use crate::ASCII_DIGITS;

#[derive(Clone)]
struct Disk {
    disk_map: Vec<usize>,
    contents: Vec<Option<usize>>,
    files: BTreeMap<usize, File>,
    free_spaces: BTreeMap<usize, usize>,
}

#[derive(Debug, Clone)]
struct File {
    position: usize,
    length: usize,
}

impl Disk {
    fn generate_contents_from_disk_map(&mut self) {
        // track the current position on disk as we iterate through free space and files
        let mut current_position = 0;
        let mut contents = Vec::new();

        for (i, size) in self.disk_map.iter().enumerate() {
            if i % 2 != 0 {
                self.free_spaces.insert(current_position, *size);
                contents.extend(vec![None; *size])
            } else {
                let block_id = if i == 0 { 0 } else { i / 2 };
                self.files.insert(
                    block_id,
                    File {
                        position: current_position,
                        length: *size,
                    },
                );
                contents.extend(vec![Some(block_id); *size])
            }
            current_position += size;
        }

        self.contents = contents;
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

    fn whole_file_compact(&mut self) {
        // Loop over the files in reverse (highest Id first)
        // call move file to get the new position
        // update files
        // move on to the next file
        let ids: Vec<usize> = self.files.keys().rev().cloned().collect();
        for id in ids {
            // Remove the file to avoid multiple mutable borrows against self
            if let Some(file) = self.files.remove(&id) {
                let new_pos = self.find_new_space_for_file(&file);
                if new_pos < file.position {
                    let free_space_size = self.free_spaces.remove(&new_pos).unwrap();
                    let new_free_pos = new_pos + file.length;
                    let new_free_size = free_space_size - file.length;
                    // println!(
                    //     "Debug: new free pos and size {}, {}",
                    //     new_free_pos, new_free_size
                    // );
                    if new_free_size > 0 {
                        self.free_spaces.insert(new_free_pos, new_free_size);
                    }

                    let new_file = File {
                        position: new_pos,
                        length: file.length,
                    };
                    self.files.insert(id, new_file);
                } else {
                    // put the file back in it's original space
                    self.files.insert(id, file);
                }
            } else {
                // println!("Warning: No file found for {id}");
            }
            // self.update_contents_from_files();
            // println!("Debug: {self}")
        }
        self.update_contents_from_files();
    }

    fn find_new_space_for_file(&self, file: &File) -> usize {
        for (&pos, &size) in &self.free_spaces {
            if size >= file.length && pos < file.position {
                return pos;
            }
        }
        file.position
    }

    fn update_contents_from_files(&mut self) {
        // Reset Contents
        // println!("Debug: Contents {self}");
        self.contents.fill(None);
        // println!("Debug: Contents {self}");

        for (id, file) in &self.files {
            let start = file.position;
            let end = start + file.length;
            // println!("Debug: File {id}: {start}, {end}");

            for pos in start..end {
                // println!("debug: position {pos}");
                self.contents[pos] = Some(*id);
            }

            // println!("Debug: Contents {self}");
        }
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
    let mut disk = Disk {
        disk_map: input
            .chars()
            .filter(|c| ASCII_DIGITS.contains(c))
            .map(|n| n.to_digit(10).unwrap() as usize) // parse().unwrap())
            .collect(),
        contents: vec![],
        files: BTreeMap::new(),
        free_spaces: BTreeMap::new(),
    };
    disk.generate_contents_from_disk_map();
    disk
}

#[aoc(day9, part1)]
fn solve_part1(input: &Disk) -> usize {
    // println!("{:?}", input.disk_map);
    // println!("{}", input);
    // println!("{:?}", input.contents);
    // println!("----");
    let mut map = input.clone();
    map.compact();
    // println!("{}", map);
    map.checksum()
}

#[aoc(day9, part2)]
fn solve_part2(input: &Disk) -> usize {
    let mut map = input.clone();
    // println!("starting contents: {:?}", map.contents);
    // println!("starting files: {:?}", map.files);
    // println!("{}", map);
    // println!("-----");
    map.whole_file_compact();
    // map.update_contents_from_files();
    // println!("{:?}", map.contents);
    // println!("{}", map);
    map.checksum()
}
