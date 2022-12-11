use std::{env, error::Error, path::Path, str::FromStr};

use aoc_common_lib::error::RuntimeError;
use aoc_common_lib::utility::read_lines;

// Override the alias to use `Box<error::Error>`.
type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, PartialEq)]
enum PlayerMove {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for PlayerMove {
    type Err = ();
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "A" => Ok(PlayerMove::Rock),
            "B" => Ok(PlayerMove::Paper),
            "C" => Ok(PlayerMove::Scissors),
            "X" => Ok(PlayerMove::Rock),
            "Y" => Ok(PlayerMove::Paper),
            "Z" => Ok(PlayerMove::Scissors),
            _ => Err(()),
        }
    }
}

#[derive(Debug, PartialEq)]
enum RoundResult {
    Lose = 0,
    Tie = 3,
    Win = 6,
}

impl FromStr for RoundResult {
    type Err = ();
    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "X" => Ok(RoundResult::Lose),
            "Y" => Ok(RoundResult::Tie),
            "Z" => Ok(RoundResult::Win),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct RoundScore {
    round: u32,
    player_1_move: PlayerMove,
    player_1_score: u32,
    player_2_move: PlayerMove,
    player_2_score: u32,
    player_2_ideal_move: PlayerMove,
    player_2_ideal_score: u32,
}

fn calculate_move_score(player_1_move: &PlayerMove, player_2_move: &PlayerMove) -> u32 {
    match player_1_move {
        PlayerMove::Rock => match player_2_move {
            PlayerMove::Rock => PlayerMove::Rock as u32 + RoundResult::Tie as u32,
            PlayerMove::Paper => PlayerMove::Rock as u32 + RoundResult::Lose as u32,
            PlayerMove::Scissors => PlayerMove::Rock as u32 + RoundResult::Win as u32,
        },
        PlayerMove::Paper => match player_2_move {
            PlayerMove::Rock => PlayerMove::Paper as u32 + RoundResult::Win as u32,
            PlayerMove::Paper => PlayerMove::Paper as u32 + RoundResult::Tie as u32,
            PlayerMove::Scissors => PlayerMove::Paper as u32 + RoundResult::Lose as u32,
        },
        PlayerMove::Scissors => match player_2_move {
            PlayerMove::Rock => PlayerMove::Scissors as u32 + RoundResult::Lose as u32,
            PlayerMove::Paper => PlayerMove::Scissors as u32 + RoundResult::Win as u32,
            PlayerMove::Scissors => PlayerMove::Scissors as u32 + RoundResult::Tie as u32,
        },
    }
}

fn calculate_ideal_move(
    player_1_move: &PlayerMove,
    player_2_ideal_result: &RoundResult,
) -> PlayerMove {
    match player_2_ideal_result {
        RoundResult::Lose => match player_1_move {
            PlayerMove::Rock => PlayerMove::Scissors,
            PlayerMove::Paper => PlayerMove::Rock,
            PlayerMove::Scissors => PlayerMove::Paper,
        },
        RoundResult::Tie => match player_1_move {
            PlayerMove::Rock => PlayerMove::Rock,
            PlayerMove::Paper => PlayerMove::Paper,
            PlayerMove::Scissors => PlayerMove::Scissors,
        },
        RoundResult::Win => match player_1_move {
            PlayerMove::Rock => PlayerMove::Paper,
            PlayerMove::Paper => PlayerMove::Scissors,
            PlayerMove::Scissors => PlayerMove::Rock,
        },
    }
}

fn parse_game_rounds(input_file_path: &str) -> Result<Vec<RoundScore>> {
    let input_file = Path::new(input_file_path);
    if !input_file.exists() {
        let error_message = format!("Path {} does not appear to exist", input_file_path);
        return Err(Box::new(RuntimeError::new(error_message)));
    }
    let mut round_scores = Vec::new();
    if let Ok(lines) = read_lines(input_file) {
        let mut round_number: u32 = 1;
        for line in lines {
            match line {
                Ok(line) => {
                    let line_parts: Vec<&str> = line.split_whitespace().collect();
                    let player_1_move = PlayerMove::from_str(line_parts[0]).unwrap();
                    let player_2_move = PlayerMove::from_str(line_parts[1]).unwrap();
                    let player_2_ideal_result = RoundResult::from_str(line_parts[1]).unwrap();
                    let player_2_ideal_move: PlayerMove =
                        calculate_ideal_move(&player_1_move, &player_2_ideal_result);

                    let player_1_score = calculate_move_score(&player_1_move, &player_2_move);
                    let player_2_score = calculate_move_score(&player_2_move, &player_1_move);
                    let player_2_ideal_score: u32 =
                        calculate_move_score(&player_2_ideal_move, &player_1_move);

                    round_scores.push(RoundScore {
                        round: round_number,
                        player_1_move,
                        player_1_score,
                        player_2_move,
                        player_2_score,
                        player_2_ideal_move,
                        player_2_ideal_score,
                    });
                    round_number += 1;
                }
                Err(err) => return Err(Box::new(err)),
            }
        }
    }

    Ok(round_scores)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Box::new(RuntimeError::new(String::from(
            "Must provide input file path",
        ))));
    }
    let input_path = &args[1];
    let round_scores = parse_game_rounds(input_path)?;

    let player_1_score_sum = round_scores
        .iter()
        .map(|round_score| round_score.player_1_score)
        .sum::<u32>();

    let player_2_score_sum = round_scores
        .iter()
        .map(|round_score| round_score.player_2_score)
        .sum::<u32>();

    let player_2_ideal_score_sum = round_scores
        .iter()
        .map(|round_score| round_score.player_2_ideal_score)
        .sum::<u32>();

    // println!("{:#?}", round_scores);

    println!(
        r#"
Player 1: {}
Player 2: {}
Player 2 Ideal Score: {}
"#,
        player_1_score_sum, player_2_score_sum, player_2_ideal_score_sum
    );

    Ok(())
}
