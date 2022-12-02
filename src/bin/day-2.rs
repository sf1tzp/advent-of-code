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

fn main() -> Result<()> {
    let input = advent_of_code_2022::read_input().expect("failed to read input");
    let mut score: usize = 0;
    for line in input.split('\n') {
        let plays: Vec<&str> = line.split(' ').collect();
        let our_move = parse_play(plays[1])?;
        let their_move = parse_play(plays[0])?;
        score += derive_score(our_move, their_move);
    }

    println!("(#1) Projected Score: {score}");

    let mut score: usize = 0;
    for line in input.split('\n') {
        let gameplan: Vec<&str> = line.split(' ').collect();
        let their_move = parse_play(gameplan[0])?;
        let outcome = parse_plan(gameplan[1])?;
        let our_move = determine_move(their_move, outcome);
        score += derive_score(our_move, their_move);
    }

    println!("(#2) Projected Score: {score}");

    Ok(())
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
