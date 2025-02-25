use std::collections::{HashMap, HashSet};

use anyhow::{anyhow, Result};

#[derive(Debug)]
struct Page {
    preceeded_by: Vec<usize>,
    followed_by: Vec<usize>,
}

#[derive(Debug)]
struct PageUpdates {
    pages: HashMap<usize, Page>,
    updates: Vec<Vec<usize>>,
}

impl PageUpdates {
    fn validate(&self, i: usize) -> bool {
        let updates = &self.updates[i];

        for (i, page_id) in updates.iter().enumerate() {
            // TODO: Avoid allocating so many sets
            let mut preceeding: HashSet<usize> = HashSet::new();
            for k in &updates[0..i] {
                preceeding.insert(*k);
            }

            let mut following: HashSet<usize> = HashSet::new();
            for k in &updates[i..updates.len()] {
                following.insert(*k);
            }

            let page = self.pages.get(page_id).unwrap();
            let mut preceeded_by: HashSet<usize> = HashSet::new();
            for k in page.preceeded_by.iter() {
                preceeded_by.insert(*k);
            }
            let mut followed_by: HashSet<usize> = HashSet::new();
            for k in page.followed_by.iter() {
                followed_by.insert(*k);
            }

            if !followed_by.is_disjoint(&preceeding) {
                return false;
            }
            if !preceeded_by.is_disjoint(&following) {
                return false;
            }
        }

        true
    }

    fn find_middle(&self, i: usize) -> Result<usize> {
        let updates = &self.updates[i];
        if updates.len() % 2 == 0 {
            return Err(anyhow!("This list should have an odd number of items.."));
        }
        Ok(updates[updates.len() / 2])
    }
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> PageUpdates {
    let input: Vec<&str> = input.split("\n\n").collect();
    assert_eq!(input.len(), 2);
    let (rules, updates) = (input[0], input[1]);

    let mut pages = HashMap::<usize, Page>::new();
    for rule in rules.lines() {
        let rule: Vec<usize> = rule
            .split('|')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        assert_eq!(rule.len(), 2);
        let (i, j) = (rule[0], rule[1]);

        let p = pages.entry(i).or_insert(Page {
            preceeded_by: vec![],
            followed_by: vec![],
        });
        p.followed_by.push(j);

        let p = pages.entry(j).or_insert(Page {
            preceeded_by: vec![],
            followed_by: vec![],
        });
        p.preceeded_by.push(i);
    }

    let mut u: Vec<Vec<usize>> = vec![];
    for update in updates.lines() {
        let update: Vec<usize> = update
            .split(',')
            .map(|c| c.parse::<usize>().unwrap())
            .collect();
        u.push(update)
    }

    PageUpdates {
        pages: pages,
        updates: u,
    }
}

#[aoc(day5, part1)]
fn solve_part1(input: &PageUpdates) -> usize {
    let mut total = 0;
    for i in 0..input.updates.len() {
        if input.validate(i) {
            match input.find_middle(i) {
                Ok(j) => total += j,
                Err(e) => panic!("Error {e}"),
            }
        }
    }

    total
}
