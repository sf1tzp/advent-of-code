use anyhow::{anyhow, Error, Result};

#[derive(Clone, Copy)]
enum Play {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

enum Outcome {
    Lose,
    Draw,
    Win,
}

#[aoc(day2, part1)]
fn solve_part1(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.lines() {
        let plays: Vec<&str> = line.split(' ').collect();
        let our_move = parse_play(plays[1]).unwrap();
        let their_move = parse_play(plays[0]).unwrap();
        score += derive_score(our_move, their_move);
    }
    score
}

#[aoc(day2, part2)]
fn solve_part2(input: &str) -> usize {
    let mut score: usize = 0;
    for line in input.lines() {
        let gameplan: Vec<&str> = line.split(' ').collect();
        let their_move = parse_play(gameplan[0]).unwrap();
        let outcome = parse_plan(gameplan[1]).unwrap();
        let our_move = determine_move(their_move, outcome);
        score += derive_score(our_move, their_move);
    }
    score
}

fn parse_play(play: &str) -> Result<Play, Error> {
    let play = play.chars().nth(0).unwrap();
    match play {
        'A' => Ok(Play::Rock),
        'B' => Ok(Play::Paper),
        'C' => Ok(Play::Scissors),
        'X' => Ok(Play::Rock),
        'Y' => Ok(Play::Paper),
        'Z' => Ok(Play::Scissors),
        _ => Err(anyhow!("could not parse {}", play)),
    }
}

fn derive_score(our_move: Play, their_move: Play) -> usize {
    match (our_move, their_move) {
        (Play::Rock, Play::Rock) => our_move as usize + 3,
        (Play::Rock, Play::Paper) => our_move as usize + 0,
        (Play::Rock, Play::Scissors) => our_move as usize + 6,

        (Play::Paper, Play::Rock) => our_move as usize + 6,
        (Play::Paper, Play::Paper) => our_move as usize + 3,
        (Play::Paper, Play::Scissors) => our_move as usize + 0,

        (Play::Scissors, Play::Rock) => our_move as usize + 0,
        (Play::Scissors, Play::Paper) => our_move as usize + 6,
        (Play::Scissors, Play::Scissors) => our_move as usize + 3,
    }
}

fn parse_plan(plan: &str) -> Result<Outcome, Error> {
    let plan = plan.chars().nth(0).unwrap();
    match plan {
        'X' => Ok(Outcome::Lose),
        'Y' => Ok(Outcome::Draw),
        'Z' => Ok(Outcome::Win),
        _ => Err(anyhow!("could not parse plan {}", plan)),
    }
}

fn determine_move(their_move: Play, outcome: Outcome) -> Play {
    match (their_move, outcome) {
        (Play::Rock, Outcome::Lose) => Play::Scissors,
        (Play::Rock, Outcome::Draw) => Play::Rock,
        (Play::Rock, Outcome::Win) => Play::Paper,

        (Play::Paper, Outcome::Lose) => Play::Rock,
        (Play::Paper, Outcome::Draw) => Play::Paper,
        (Play::Paper, Outcome::Win) => Play::Scissors,

        (Play::Scissors, Outcome::Lose) => Play::Paper,
        (Play::Scissors, Outcome::Draw) => Play::Scissors,
        (Play::Scissors, Outcome::Win) => Play::Rock,
    }
}
