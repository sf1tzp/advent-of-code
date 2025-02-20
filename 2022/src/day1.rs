type Calories = usize;

struct Elf {
    food: Vec<Calories>,
}

// Split the input on blank lines, then parse each elf's inventory
#[aoc_generator(day1)]
pub fn get_calorie_counts(input: &str) -> Vec<Calories> {
    let mut calorie_counts: Vec<Calories> = input
        .split("\n\n")
        .map(|inv| Elf {
            food: inv
                .split('\n')
                .map(|food| {
                    food.parse::<Calories>()
                        .unwrap_or_else(|_| panic!("failed to parse calories from {}", food))
                })
                .collect(),
        })
        .map(|elf| elf.food.iter().sum())
        .collect();
    calorie_counts.sort_by(|a, b| b.cmp(a));
    calorie_counts
}

#[aoc(day1, part1)]
pub fn solve_part1(calorie_counts: &[Calories]) -> usize {
    calorie_counts[0]
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Calories]) -> Calories {
    let top_3 = &input[0..3];
    let top_3_sum = top_3.iter().sum();
    top_3_sum
}
