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
    if i == 0 {
        return vec![1];
    }
    let digits = count_digits(i);
    if digits % 2 == 0 {
        // split the number at the 'halfway point'
        let divisor = 10usize.pow(digits as u32 / 2);
        let lhs = i / divisor;
        let rhs = i % divisor;
        return vec![lhs, rhs];
    }
    return vec![i * 2024];
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
        rocks = rocks.iter().map(|x| process(*x)).flatten().collect();
        // println!("{:?}", rocks);
    }
    rocks.len()
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
