use std::collections::VecDeque;

use itertools::Itertools;

#[derive(Debug)]
struct TestCase {
    expected: usize,
    inputs: Vec<usize>,
}

#[derive(Debug, Clone, PartialEq)]
enum Symbol {
    Term(usize),
    Add,
    Multiply,
}

#[aoc_generator(day7)]
fn parse_input(input: &str) -> Vec<TestCase> {
    let mut tests = vec![];
    for line in input.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        assert_eq!(parts.len(), 2);
        let expected: usize = parts[0].parse().unwrap();
        let inputs: Vec<usize> = parts[1]
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        tests.push(TestCase { expected, inputs });
    }

    tests
}

fn get_equation_permutations(test_case: &TestCase) -> Vec<Vec<Symbol>> {
    let terms: Vec<Symbol> = test_case.inputs.iter().map(|i| Symbol::Term(*i)).collect();

    match terms.len() {
        0 => return vec![],
        1 => return vec![terms],
        _ => {}
    };

    // Generate all possible combinations of operations
    let operations = [Symbol::Add, Symbol::Multiply];
    let op_combinations = (0..terms.len() - 1)
        .map(|_| operations.iter().cloned())
        .multi_cartesian_product();

    // Interleave terms and operations for each combination
    op_combinations
        .map(|ops| {
            // Start with first term
            let mut equation = vec![terms[0].clone()];

            // Add each operation followed by the next term
            for (i, op) in ops.into_iter().enumerate() {
                equation.push(op);
                equation.push(terms[i + 1].clone());
            }

            equation
        })
        .collect()
}

// TODO: Fix me :)
// Use a deque to pop instead of taking 3 next()s
fn check_equation(expected: usize, equation: &Vec<Symbol>) -> bool {
    let mut stack: VecDeque<Symbol> = VecDeque::from(equation.clone());

    // Variables to store data off the stack
    let mut lhs: Option<usize> = None;
    let mut op: Option<Symbol> = None;
    let mut rhs: Option<usize> = None;

    // Loop over the stack until all items have been popped,
    // populating the storage variables as we go
    while let Some(x) = stack.pop_front() {
        match x {
            Symbol::Term(i) => {
                if lhs == None {
                    lhs = Some(i)
                } else {
                    rhs = Some(i)
                }
            }
            Symbol::Add => op = Some(Symbol::Add),
            Symbol::Multiply => op = Some(Symbol::Multiply),
        }

        // Once a full expression has been loaded (term, op, term)
        // Calculate the result, and push it onto the stack.
        if lhs != None && op != None && rhs != None {
            let mut result = 0;
            match op.unwrap() {
                Symbol::Add => result = lhs.unwrap() + rhs.unwrap(),
                Symbol::Multiply => result = lhs.unwrap() * rhs.unwrap(),
                _ => panic!("operation was a term!"),
            }
            let result = Symbol::Term(result);
            stack.push_front(result);

            // Reset the storage variables for the next expression
            lhs = None;
            op = None;
            rhs = None;
        }
    }

    assert_ne!(lhs, None);
    lhs.unwrap() == expected
}

fn check_permutations(expected: usize, permutations: Vec<Vec<Symbol>>) -> bool {
    // Loop through each permutation
    for equation in permutations.iter() {
        if check_equation(expected, equation) {
            return true;
        }
    }
    false
}

#[aoc(day7, part1)]
fn solve_part1(input: &Vec<TestCase>) -> usize {
    let mut total = 0;
    for t in input.iter() {
        let p = get_equation_permutations(&t);
        // println!("{:?}", p);
        if check_permutations(t.expected, p) {
            total += t.expected
        }
    }
    total
}

#[cfg(test)]
mod test {
    #[test]
    fn enum_loop() {}
}
