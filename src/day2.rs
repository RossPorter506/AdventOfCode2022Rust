use crate::prelude::*;
use std::cmp::Ordering;

// Given a game of rock-paper-scissors given as "{A,B,C} {X,Y,Z}" (e.g. "A Z"), calculate
// the score of each game and return the total score over all games.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let mut total_score = 0usize;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {continue}

        let game: Vec<&str> = line.split(' ').collect();
        let opponent_pick: RPS = game[0].try_into()?;
        let my_pick: RPS = game[1].try_into()?;
        let score = score_game(my_pick, opponent_pick);
        total_score += score;
    }
    Ok(total_score)
}

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
enum RPS {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
} use RPS::*;
impl TryFrom<&str> for RPS {
    type Error = anyhow::Error;
    fn try_from(str: &str) -> Result<RPS> {
        match str {
            "A" | "X" => Ok(RPS::Rock),
            "B" | "Y" => Ok(RPS::Paper),
            "C" | "Z" => Ok(RPS::Scissors),
            _ => Err(anyhow!("Invalid char!"))
        }
    }
}

/// A non-transitive total order. 
/// Reflexive, antisymmetric, strongly-connected.
trait NonTransitiveOrder {
    fn cmp(&self, other: &Self) -> Ordering;
}
impl NonTransitiveOrder for RPS {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Rock, Paper) =>        Ordering::Less,
            (Rock, Scissors) =>     Ordering::Greater,
            (Paper, Scissors) =>    Ordering::Less,
            (Paper, Rock) =>        Ordering::Greater,
            (Scissors, Rock) =>     Ordering::Less,
            (Scissors, Paper) =>    Ordering::Greater,
            _ =>                    Ordering::Equal,
        }
    }
}

/// Produce a score for a game of rock-paper-scissors.
fn score_game(player_pick: RPS, opp_pick: RPS) -> usize {
    let mut score = player_pick as usize;
    score += match player_pick.cmp(&opp_pick) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    };
    score
}

/********** Part 2 begins **********/

// Turns out the second column is the required game result, not your move.
// Same as above, but instead calculate my_pick from the opponent's move and required outcome.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day2.txt")?;
    let reader = BufReader::new(file);
    let mut total_score = 0usize;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {continue}

        let game: Vec<&str> = line.split(' ').collect();
        let opponent_pick: RPS = game[0].try_into()?;
        let required_outcome: GameResult = game[1].try_into()?;
        let my_pick = get_symbol_for_outcome(opponent_pick, required_outcome);
        total_score += score_game(my_pick, opponent_pick);
    }
    Ok(total_score)
}

#[derive(PartialEq, Eq, Copy, Clone)]
enum GameResult {
    Win,
    Draw,
    Loss,
} use GameResult::*;
impl TryFrom<&str> for GameResult {
    type Error = anyhow::Error;
    fn try_from(str: &str) -> Result<GameResult> {
        match str {
            "X" => Ok(Loss),
            "Y" => Ok(Draw),
            "Z" => Ok(Win),
            _ => Err(anyhow!("Invalid char!"))
        }
    }
}

/// Given your opponent's move and the required outcome, determine what move to play.
fn get_symbol_for_outcome(opp_pick: RPS, outcome: GameResult) -> RPS {
    match outcome {
        Win => get_winning_move(opp_pick),
        Draw => opp_pick,
        Loss => get_losing_move(opp_pick),
    }
}

fn get_losing_move(opp_pick: RPS) -> RPS {
    match opp_pick {
        Rock => Scissors,
        Paper => Rock,
        Scissors => Paper,
    }
}

fn get_winning_move(opp_pick: RPS) -> RPS {
    match opp_pick {
        Rock => Paper,
        Paper => Scissors,
        Scissors => Rock,
    }
}