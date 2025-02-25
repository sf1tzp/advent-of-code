use std::cmp::Ordering;

struct Lists {
    left: Vec<usize>,
    right: Vec<usize>,
}

#[aoc_generator(day1)]
fn split_lists(input: &str) -> Lists {
    // Read the input
    // Each line contains two numbers separated by spaces
    // Assemble the two lists into a the left and right fields of a List struct
    let mut left = Vec::new();
    let mut right = Vec::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let mut nums = line.split_whitespace();
        left.push(nums.next().unwrap().parse().unwrap());
        right.push(nums.next().unwrap().parse().unwrap());
    }

    // Sort the lists
    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());
    Lists { left, right }
}

#[aoc(day1, part1)]
fn solve_part1(input: &Lists) -> usize {
    let mut sum = 0;
    for i in 0..input.left.len() {
        match input.left[i].cmp(&input.right[i]) {
            Ordering::Less => sum += input.right[i] - input.left[i],
            Ordering::Equal => (),
            Ordering::Greater => sum += input.left[i] - input.right[i],
        }
    }
    sum
}

#[aoc(day1, part2)]
fn solve_part2(input: &Lists) -> usize {
    let mut sum = 0;
    // condense the right list into a hashmap where the key is the number and the value is the number of times it appears
    let mut right_map = std::collections::HashMap::new();
    for num in &input.right {
        *right_map.entry(num).or_insert(0) += 1;
    }

    for num in &input.left {
        if let Some(count) = right_map.get(num) {
            // println!("{} found {} times", num, count);
            sum += num * count;
            // println!("sum is now {}", sum);
        }
    }

    sum
}
