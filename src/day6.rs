use std::collections::{HashSet, VecDeque};

static _EASY: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz"; // Should result in 5

struct Identifier {
    buffer: VecDeque<char>,
}

impl Identifier {
    fn new() -> Identifier {
        Identifier {
            buffer: VecDeque::new(),
        }
    }
    
    fn push(&mut self, item: char) {
        self.buffer.push_back(item);
        if self.buffer.len() > 4 {
            self.buffer.pop_front();
        }
    }

    fn check_for_start_of_packet(&self) -> bool {
        // println!("checking buffer {:?}", self.buffer);
        let mut seen: HashSet<char> = HashSet::new();
        if self.buffer.len() != 4 {
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

fn find_packet(input: &str) -> Option<usize> {
    let mut id = Identifier::new();
    for (i, c) in input.chars().enumerate() {
        println!("{} {}", i, c);
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
    find_packet(input).unwrap() + 1
}
