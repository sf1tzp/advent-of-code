struct Record {
    readings: Vec<usize>,
}

#[aoc_generator(day2)]
fn parse_input(input: &str) -> Vec<Record> {
    input
        .lines()
        .map(|line| {
            let nums = line.split_whitespace();
            let nums = nums.map(|num| num.parse().unwrap());
            Record {
                readings: nums.collect(),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn solve_part1(input: &[Record]) -> usize {
    let mut count = 0;
    for record in input {
        if validate_readings(&record.readings) {
            count += 1;
        }
    }
    count
}

fn validate_readings(readings: &[usize]) -> bool {
    let mut increasing = true;
    let mut decreasing = true;

    for i in 0..readings.len() - 1 {
        let j = readings[i];
        let k = readings[i + 1];

        let unsafe_level = unsafe_change(j, k);
        if j < k {
            decreasing = false;
        } else if j > k {
            increasing = false;
        }

        if unsafe_level || (!increasing && !decreasing) {
            return false;
        }
    }
    true
}

fn unsafe_change(i: usize, j: usize) -> bool {
    let magnitude = i as isize - j as isize;
    magnitude == 0 || magnitude.abs() > 3
}

// #[aoc(day2, part2)]
// pub fn solve_part2(input: &[Record]) -> usize {
//     let mut count = 0;
//     for record in input {
//         // if validate_readings(&record.readings, Some(1)) {
//         //     println!("{:?} is safe", record.readings);
//         //     count += 1;
//         // } else {
//         //     println!("{:?} is unsafe", record.readings);
//         // }
//     }
//     count
// }
