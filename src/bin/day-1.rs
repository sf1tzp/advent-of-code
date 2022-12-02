type Calories = isize;

#[derive(Debug)]
struct Elf {
    food: Vec<Calories>,
}

fn main() {
    let input = advent_of_code_2022::read_input().expect("failed to read input");
    let inventory = take_inventory(input);

    let mut calorie_counts = inventory
        .iter()
        .map(|elf| elf.food.iter().sum())
        .collect::<Vec<Calories>>();

    calorie_counts.sort_by(|a, b| b.cmp(a));

    let max = calorie_counts[0];
    println!("(#1) Most Calories: {max}");

    let top_3 = &calorie_counts[0..3];
    let top_3_sum: Calories = top_3.iter().sum();
    println!(
        "(#2) The Top 3 Have {:?} and Their Sum is {}",
        top_3, top_3_sum
    )
}

// Split the input on blank lines, then parse each elf's inventory
fn take_inventory(mut input: String) -> Vec<Elf> {
    // Can't decide how I feel about this code. But it was fun to write, kinda :)
    let inventory: Vec<Elf> = input
        .split("\n\n")
        .map(|inv| Elf {
            food: inv
                .split('\n')
                .map(|food| {
                    food.parse::<Calories>()
                        .expect(&format!("failed to parse calories from {}", food))
                })
                .collect(),
        })
        .collect();

    println!("Took inventories of {} elves", inventory.len());
    inventory
}
