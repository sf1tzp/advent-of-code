use std::{collections::VecDeque, vec};
use std::cmp::Ordering;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    Number(usize),
    List(Vec<Data>),
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = self;
        let rhs = other;
        match (self, other) {
            (Data::Number(l), Data::Number(r)) => l.cmp(r),
            (Data::List(l), Data::List(r)) => {
                let mut l_iter = l.iter();
                let mut r_iter = r.iter();
                loop {
                    let (l, r) = (l_iter.next(), r_iter.next());
                    match (l, r) {
                        (Some(l), Some(r)) => {
                            let cmp = l.cmp(r);
                            if cmp != Ordering::Equal {
                                return cmp;
                            }
                        }
                        (Some(_), None) => {
                            return Ordering::Greater;
                        }
                        (None, Some(_)) => {
                            return Ordering::Less;
                        }
                        (None, None) => {
                            return Ordering::Equal;
                        }
                    }
                }
            },
            (Data::Number(l), Data::List(_)) => {
                let lhs = Data::List(vec![Data::Number(*l)]);
                lhs.cmp(rhs)
            }
            (Data::List(_), Data::Number(r)) => {
                let rhs = Data::List(vec![Data::Number(*r)]);
                lhs.cmp(&rhs)
            }
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Packet {
    data: Vec<Data>,
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let lhs = &self.data;
        let rhs = &other.data;
        let mut l_iter = lhs.iter();
        let mut r_iter = rhs.iter();
        loop {
            let (l, r) = (l_iter.next(), r_iter.next());
            match (l, r) {
                (Some(l), Some(r)) => {
                    let cmp = l.cmp(r);
                    if cmp != Ordering::Equal {
                        return cmp;
                    }
                }
                (Some(_), None) => {
                    return Ordering::Greater;
                }
                (None, Some(_)) => {
                    return Ordering::Less;
                }
                (None, None) => {
                    return Ordering::Equal;
                }
            }
        }
    }
}
impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
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
fn solve_part1(packets: &[(Packet, Packet)]) -> usize {
    let mut count = 0;
    for (i, (lhs, rhs)) in packets.iter().enumerate() {
        if lhs < rhs {
            count += i + 1;
        }
    }
    count
}

#[aoc(day13, part2)]
fn solve_part2(packets: &[(Packet, Packet)]) -> usize {
    let mut packets = packets.iter()
        .flat_map(|(p1, p2)| vec![p1, p2])
        .collect::<Vec<&Packet>>();
    let dividers = vec![
        Packet::read("[[2]]").unwrap(),
        Packet::read("[[6]]").unwrap(),
    ];
    packets.extend(dividers.iter());
    packets.sort();

    let mut key = 1;
    for (i, packet) in packets.iter().enumerate() {
        if dividers.contains(packet) {
            key *= i + 1;
        }
    }
    key
}

#[cfg(test)]
mod tests {
    use std::vec;

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

    #[test]
    fn test_ord_for_data() {
        struct TestCase {
            description: &'static str,
            lhs: Data,
            rhs: Data,
            expected: Ordering,
        }

        let cases = vec![
            TestCase {
                description: "Number < Number",
                lhs: Data::Number(1),
                rhs: Data::Number(2),
                expected: Ordering::Less,
            },
            TestCase {
                description: "Number > Number",
                lhs: Data::Number(2),
                rhs: Data::Number(1),
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Number == Number",
                lhs: Data::Number(1),
                rhs: Data::Number(1),
                expected: Ordering::Equal,
            },
            TestCase {
                description: "Equal Lists",
                lhs: Data::List(vec![Data::Number(1)]),
                rhs: Data::List(vec![Data::Number(1)]),
                expected: Ordering::Equal,
            },
            TestCase {
                description: "List < List",
                lhs: Data::List(vec![Data::Number(1)]),
                rhs: Data::List(vec![Data::Number(2)]),
                expected: Ordering::Less,
            },
            TestCase {
                description: "List > List",
                lhs: Data::List(vec![Data::Number(2)]),
                rhs: Data::List(vec![Data::Number(1)]),
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Shorter List < Longer List",
                lhs: Data::List(vec![Data::Number(1)]),
                rhs: Data::List(vec![Data::Number(1), Data::Number(2)]),
                expected: Ordering::Less,
            },
            TestCase {
                description: "Longer List > Shorter List",
                lhs: Data::List(vec![Data::Number(1), Data::Number(2)]),
                rhs: Data::List(vec![Data::Number(1)]),
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Number == One element list",
                lhs: Data::Number(1),
                rhs: Data::List(vec![Data::Number(1)]),
                expected: Ordering::Equal,
            },
            TestCase {
                description: "Number < Two element list",
                lhs: Data::Number(1),
                rhs: Data::List(vec![Data::Number(1), Data::Number(2)]),
                expected: Ordering::Less,
            },
            TestCase {
                description: "Number > Two element list",
                lhs: Data::Number(3),
                rhs: Data::List(vec![Data::Number(1), Data::Number(2)]),
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Empty List < List",
                lhs: Data::List(vec![]),
                rhs: Data::List(vec![Data::Number(1)]),
                expected: Ordering::Less,
            },
            TestCase {
                description: "Empty List < Number ",
                lhs: Data::List(vec![]),
                rhs: Data::Number(1),
                expected: Ordering::Less,
            },
        ];

        for case in cases {
            let result = case.lhs.cmp(&case.rhs);
            assert_eq!(result, case.expected, "{}", case.description);
        }
    }

    #[test]
    fn test_ord_for_packet() {
        struct TestCase {
            description: &'static str,
            lhs: &'static str,
            rhs: &'static str,
            expected: Ordering,
        }
        let cases = vec![
            TestCase {
                description: "Equal Packets",
                lhs: "[1]",
                rhs: "[1]",
                expected: Ordering::Equal,
            },
            TestCase {
                description: "Packet < Packet",
                lhs: "[1]",
                rhs: "[2]",
                expected: Ordering::Less,
            },
            TestCase {
                description: "Example 1",
                lhs: "[1,1,3,1,1]",
                rhs: "[1,1,5,1,1]",
                expected: Ordering::Less,
            },
            TestCase {
                description: "Example 2",
                lhs: "[[1],[2,3,4]]",
                rhs: "[[1],4]",
                expected: Ordering::Less,
            },
            TestCase {
                description: "Example 3",
                lhs: "[9]",
                rhs: "[[8,7,6]]",
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Example 4",
                lhs: "[[4,4],4,4]",
                rhs: "[[4,4],4,4,4]",
                expected: Ordering::Less,
            },
            TestCase {
                description: "Example 5",
                lhs: "[7,7,7,7]",
                rhs: "[7,7,7]",
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Example 6",
                lhs: "[]",
                rhs: "[3]",
                expected: Ordering::Less,
            },
            TestCase {
                description: "Example 7",
                lhs: "[[[]]]",
                rhs: "[[]]",
                expected: Ordering::Greater,
            },
            TestCase {
                description: "Example 8",
                lhs: "[1,[2,[3,[4,[5,6,7]]]],8,9]",
                rhs: "[1,[2,[3,[4,[5,6,0]]]],8,9]",
                expected: Ordering::Greater,
            },
        ];
        for case in cases {
            let lhs = Packet::read(case.lhs).unwrap();
            let rhs = Packet::read(case.rhs).unwrap();
            let result = lhs.cmp(&rhs);
            assert_eq!(result, case.expected, "{}", case.description);
        }
    }
}
