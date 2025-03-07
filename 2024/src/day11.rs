use std::collections::HashMap;

fn count_digits(mut i: usize) -> usize {
    if i == 0 {
        return 1;
    }
    let mut count = 0;
    while i > 0 {
        i /= 10;
        count += 1;
    }
    count
}

fn process(i: usize) -> Vec<usize> {
    let digits = count_digits(i);
    if digits % 2 == 0 {
        // split the number at the 'halfway point'
        let divisor = 10usize.pow(digits as u32 / 2);
        let lhs = i / divisor;
        let rhs = i % divisor;
        vec![lhs, rhs]
    } else if i == 0 {
        vec![1]
    } else {
        vec![i * 2024]
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> Vec<usize> {
    input
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect()
}

#[aoc(day11, part1)]
fn solve_part1(input: &Vec<usize>) -> usize {
    let mut rocks = input.clone();
    for _ in 0..25 {
        // Too slow and memory intensive for large lists...
        rocks = rocks.iter().map(|x| process(*x)).flatten().collect();
    }
    rocks.len()
}

// Loop through the input counter
// process each stone - the result doesn't change for the same input
// so we'll increment the output counter by n for each result
fn update_counts(input: HashMap<usize, usize>) -> HashMap<usize, usize> {
    let mut output = HashMap::new();
    for (stone, n) in input {
        for result in process(stone) {
            output.entry(result).and_modify(|x| *x += n).or_insert(n);
        }
    }
    output
}

#[aoc(day11, part2)]
fn solve_part2(input: &Vec<usize>) -> usize {
    // Since order doesn't actually matter, we'll simply keep track of how many times we each number
    let mut counter = HashMap::new();
    for i in input {
        counter.entry(*i).or_insert(1);
    }

    for _ in 0..75 {
        counter = update_counts(counter);
    }
    counter.values().sum()
}

#[cfg(test)]
mod test {
    use crate::day11::count_digits;

    #[test]
    fn test_count_digits() {
        assert_eq!(count_digits(0), 1);
        assert_eq!(count_digits(1), 1);
        assert_eq!(count_digits(10), 2);
        assert_eq!(count_digits(1000), 4);
    }
}
