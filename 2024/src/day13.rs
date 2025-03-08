use once_cell::sync::Lazy;
use regex::Regex;

#[derive(Debug)]
struct Machine {
    a_button: (isize, isize),
    b_button: (isize, isize),
    prize_location: (isize, isize),
}

fn capture(line: &str) -> (isize, isize) {
    static PATTERN: &str = r"X.(?<x>\d+).+Y.(?<y>\d+)";
    static RE: Lazy<Regex> = Lazy::new(|| Regex::new(PATTERN).expect("invalid regex"));
    let foo: Vec<(isize, isize)> = RE
        .captures_iter(line)
        .map(|c| {
            assert!(c.len() == 3);
            let x = c["x"].parse::<isize>().unwrap();
            let y = c["y"].parse::<isize>().unwrap();
            (x, y)
        })
        .collect();
    assert!(foo.len() == 1);
    foo[0]
}

fn solve(input: &Machine) -> Option<(isize, isize)> {
    // x-axis coordinates
    let a = input.a_button.0;
    let b = input.b_button.0;
    let c = input.prize_location.0;

    // y-axis coordinates
    let d = input.a_button.1;
    let e = input.b_button.1;
    let f = input.prize_location.1;

    // ax + by = c
    // dx + ey = f
    //
    // Solve for x:
    // multiply the top by e, and the bottom by b
    // aex + bey = ce
    // bdx + bey = bf
    //
    // subtract
    // (ae - bd)x = ce - bf
    //
    // divide
    // x = (ce - bf) / (ae - bd)
    let numerator = (c * e) - (b * f);
    let denomenator = (a * e) - (b * d);
    // Check for a remainder by using %
    // The compiler actually optimizes this operation to avoid multiple divisions :)
    let (x, remainder) = (numerator / denomenator, numerator % denomenator);
    if remainder != 0 {
        return None;
    }

    // Solve for y:
    // y = (c - ax) / b
    let numerator = c - (a * x);
    let (y, remainder) = (numerator / b, numerator % b);
    if remainder != 0 {
        return None;
    }

    assert_eq!(a * x + b * y, c);

    Some((x, y))
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = vec![];
    let mut lines = input.lines().peekable();
    while lines.peek().is_some() {
        let m = Machine {
            a_button: capture(lines.next().unwrap()),
            b_button: capture(lines.next().unwrap()),
            prize_location: capture(lines.next().unwrap()),
        };
        let _ = lines.next(); // Skip the blank line separating input blocks
        machines.push(m);
    }

    machines
}

#[aoc(day13, part1)]
fn solve_part1(input: &Vec<Machine>) -> isize {
    let mut count = 0;
    for machine in input {
        // println!("{:?}", machine);
        if let Some(result) = solve(&machine) {
            if result.0 > 100 || result.1 > 100 {
                panic!("this game is a scam!")
            }
            count += result.0 * 3;
            count += result.1;
        }
    }

    count
}

#[aoc(day13, part2)]
fn solve_part2(input: &Vec<Machine>) -> isize {
    let mut count = 0;
    for machine in input {
        let offset = 10000000000000;
        let machine = Machine {
            a_button: machine.a_button,
            b_button: machine.b_button,
            prize_location: (
                machine.prize_location.0 + offset,
                machine.prize_location.1 + offset,
            ),
        };
        if let Some(result) = solve(&machine) {
            count += result.0 * 3;
            count += result.1;
        }
    }

    count
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_solve() {
        let machine = Machine {
            a_button: (94, 34),
            b_button: (22, 67),
            prize_location: (8400, 5400),
        };
        let result = solve(&machine);
        assert!(result.is_some());
        let result = result.unwrap();
        assert_eq!(result.0, 80);
        assert_eq!(result.1, 40);
    }
}
