use std::collections::{HashSet, VecDeque};

static _EASY: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz"; // Should result in 5

#[derive(Clone, Copy)]
enum Marker {
    PacketStart = 4,
    MessageStart = 14,
}

struct Identifier {
    class: Marker,
    buffer: VecDeque<char>,
}

impl Identifier {
    fn new(class: Marker) -> Identifier {
        Identifier {
            class,
            buffer: VecDeque::new(),
        }
    }
    
    fn push(&mut self, item: char) {
        self.buffer.push_back(item);
        if self.buffer.len() > self.class as usize {
            self.buffer.pop_front();
        }
    }

    fn check_for_start_of_packet(&self) -> bool {
        // println!("checking buffer {:?}", self.buffer);
        let mut seen: HashSet<char> = HashSet::new();
        if self.buffer.len() != self.class as usize {
            return false;
        }
        for c in self.buffer.iter() {
            if seen.contains(c) {
                return false;
            } else {
                seen.insert(*c);
            }
        }
        true
    }
}

fn find_marker(input: &str, class: Marker) -> Option<usize> {
    let mut id = Identifier::new(class);
    for (i, c) in input.chars().enumerate() {
        id.push(c);
        if id.check_for_start_of_packet() {
            return Some(i);
        }
    }

    None
}

#[aoc(day6, part1)]
fn solve_part1(input: &str) -> usize {
    // let input = EASY;
    find_marker(input, Marker::PacketStart).unwrap() + 1
}

#[aoc(day6, part2)]
fn solve_part2(input: &str) -> usize {
    find_marker(input, Marker::MessageStart).unwrap() + 1
}