use std::{
    env,
    fs::File,
    io::{self, BufReader, Read},
};

extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;
pub mod day8;

pub mod day11;

pub static ASCII_DIGITS: [char; 10] = ['0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];

pub static ASCII_LOWERCASE: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

pub static ASCII_UPPERCASE: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

pub fn read_input() -> io::Result<String> {
    let input_path = env::var("INPUT_PATH").expect("INPUT_PATH not defined");
    let file = File::open(&input_path).expect(&format!("Input file {} not found", input_path));

    let mut reader = BufReader::new(file);
    let mut input = String::new();

    reader.read_to_string(&mut input)?;

    // Remove Trailing Newline
    if input.ends_with('\n') {
        input.pop();
    }

    Ok(input)
}

aoc_lib! { year = 2022 }
