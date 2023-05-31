use std::collections::VecDeque;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    Number(usize),
    List(Vec<Data>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    data: Vec<Data>,
}

impl Packet {
    #[allow(clippy::while_let_on_iterator)]
    fn read(line: &str) -> Result<Packet> {
        let mut packet = Packet {
            data: Vec::<Data>::new(),
        };
        let mut stack = VecDeque::<Data>::new();

        if line.is_empty() {
            return Err(anyhow!("Empty line"));
        }

        // loop over line, pushing and popping from stack
        // When a square bracket is encountered, push a new Data onto the stack
        // When a number is encountered, append it to the last Data in the stack
        // When a closing brack is encountered, pop the last Data from the stack and append it to the previous Data in the stack
        let mut iterator = line.chars().peekable();
        while let Some(c) = iterator.next() {
            // exit if this line is empty
            match c {
                '[' => {
                    let data = Data::List(Vec::<Data>::new());
                    stack.push_back(data);
                }
                '0'..='9' => {
                    // Need to handle multi digit numbers
                    // Read all digits until we hit a comma or closing bracket
                    let mut chars = Vec::<char>::new();
                    chars.push(c);
                    while let Some(b) = iterator.peek() {
                        match b {
                            ',' | ']' => {
                                break;
                            }
                            '0'..='9' => {
                                let b = iterator.next();
                                chars.push(b.unwrap());
                            }
                            _ => return Err(anyhow!("Unexpected character: {}", b)),
                        }
                    }

                    let number = chars.iter().collect::<String>().parse::<usize>().unwrap();
                    let data = Data::Number(number);

                    match stack.back_mut() {
                        Some(Data::List(list)) => {
                            list.push(data);
                        }
                        _ => {
                            return Err(anyhow!("Expected a list on the stack"));
                        }
                    }
                }
                ',' => {
                    continue;
                }
                ']' => {
                    let data = stack.pop_back().unwrap();
                    match stack.back_mut() {
                        Some(Data::List(list)) => {
                            list.push(data);
                        }
                        Some(Data::Number(_)) => {
                            return Err(anyhow!("Expected a list on the stack"));
                        }
                        None => {
                            stack.push_back(data);
                        }
                    }
                }
                _ => {
                    return Err(anyhow!("Unexpected character: {}", c));
                }
            }
        }
        stack.into_iter().for_each(|d| packet.data.push(d));

        Ok(packet)
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> Vec<(Packet, Packet)> {
    // Read the input line by line, parsing each line into a Packet
    // Then, for every pair of Packets, create a tuple
    // Then collect into a Vec<(Packet, Packet)>
    let packets = input
        .lines()
        .filter_map(|line| {
            match Packet::read(line) {
                Ok(packet) => Some(packet),
                Err(_) => None, // TODO: Check if this is a panic-worthy error instead of assuming no packet
            }
        })
        .collect::<Vec<Packet>>();

    packets
        .chunks(2)
        .map(|chunk| {
            if chunk.len() != 2 {
                panic!("Expected a chunk of 2 packets, got {:?}", chunk.len());
            }
            let lhs = chunk[0].clone();
            let rhs = chunk[1].clone();
            (lhs, rhs)
        })
        .collect()
}

#[aoc(day13, part1)]
fn solve_part1(packets: &Vec<(Packet, Packet)>) -> usize {
    for (lhs, rhs) in packets {
        println!("lhs: {:?}", lhs);
        println!("rhs: {:?}", rhs);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_packet() {
        struct TestCase {
            description: &'static str,
            input: &'static str,
            expected: Packet,
            want_err: bool,
        }
        let cases: Vec<TestCase> = vec![
            TestCase {
                description: "Empty",
                input: "",
                expected: Packet { data: vec![] },
                want_err: true,
            },
            TestCase {
                description: "Basic",
                input: "[1]",
                expected: Packet {
                    data: vec![Data::List(vec![Data::Number(1)])],
                },
                want_err: false,
            },
            TestCase {
                description: "two nested lists",
                input: "[[1],[2]]",
                expected: Packet {
                    data: vec![Data::List(vec![
                        Data::List(vec![Data::Number(1)]),
                        Data::List(vec![Data::Number(2)]),
                    ])],
                },
                want_err: false,
            },
            TestCase {
                description: "double nested lists",
                input: "[[1],[[2]]]",
                expected: Packet {
                    data: vec![Data::List(vec![
                        Data::List(vec![Data::Number(1)]),
                        Data::List(vec![Data::List(vec![Data::Number(2)])]),
                    ])],
                },
                want_err: false,
            },
            TestCase {
                description: "empty nested lists",
                input: "[[],[]]",
                expected: Packet {
                    data: vec![Data::List(vec![Data::List(vec![]), Data::List(vec![])])],
                },
                want_err: false,
            },
            TestCase {
                description: "complex nested lists with multi digit numbers including 10",
                input: "[[1,2,3,4,5],[],6,[[7,8,9,10]]]",
                expected: Packet {
                    data: vec![Data::List(vec![
                        Data::List(vec![
                            Data::Number(1),
                            Data::Number(2),
                            Data::Number(3),
                            Data::Number(4),
                            Data::Number(5),
                        ]),
                        Data::List(vec![]),
                        Data::Number(6),
                        Data::List(vec![Data::List(vec![
                            Data::Number(7),
                            Data::Number(8),
                            Data::Number(9),
                            Data::Number(10),
                        ])]),
                    ])],
                },
                want_err: false,
            },
        ];
        for case in cases {
            let result = Packet::read(case.input);
            if case.want_err {
                assert!(
                    result.is_err(),
                    "{} did not get expected error",
                    case.description
                );
            } else {
                assert_eq!(
                    result.unwrap(),
                    case.expected,
                    "{} did not get expected output",
                    case.description,
                );
            }
        }
    }

    #[test]
    fn test_input_generator() {
        // Test that we can read pairs of packets separated by newlines
        let input = "[1]\n\
                     [1]\n\
                     \n\
                     [2]\n\
                     [2]\n\\";
        let expected = vec![
            (
                Packet {
                    data: vec![Data::List(vec![Data::Number(1)])],
                },
                Packet {
                    data: vec![Data::List(vec![Data::Number(1)])],
                },
            ),
            (
                Packet {
                    data: vec![Data::List(vec![Data::Number(2)])],
                },
                Packet {
                    data: vec![Data::List(vec![Data::Number(2)])],
                },
            ),
        ];
        let result = input_generator(input);
        assert_eq!(result, expected, "Packets should match");
    }
}
