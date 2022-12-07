use super::solve::Solution;
use std::path::Path;

pub struct DayTwo {}

impl Solution for DayTwo {
    type Ret = (u32, u32);
    type Converted = Vec<String>;

    fn solve() -> Self::Ret {
        let input =
            Self::get_input(&Path::new("static/input_two.txt")).expect("Failed to get static file");

        let converted = Self::convert(&input);

        let score_cheatmode_selection = DayTwo::calc_total_score(&converted, CheatMode::Selection);
        let score_cheatmode_outcome = DayTwo::calc_total_score(&converted, CheatMode::Outcome);

        (score_cheatmode_selection, score_cheatmode_outcome)
    }

    fn convert(input: &str) -> Self::Converted {
        input
            .split("\n")
            .map(|s| String::from(s))
            .collect::<Vec<String>>()
    }
}

#[derive(PartialEq)]
pub enum CheatMode {
    Outcome,
    Selection,
}

const ROCK: u32 = 1;
const PAPER: u32 = 2;
const SCISSORS: u32 = 3;

const WIN: u32 = 6;
const DRAW: u32 = 3;

impl DayTwo {
    // calculate total score based on player one selection and player two selection
    pub fn calc_total_score(input: &Vec<String>, cheat_mode: CheatMode) -> u32 {
        input.iter().fold(0, |mut acc, round| {
            let [player_one, player_two]: [&str; 2] = round
                .split_whitespace()
                .collect::<Vec<&str>>()
                .try_into()
                .unwrap();

            match player_one {
                "A" if player_two == "X" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += ROCK + DRAW;
                    } else {
                        acc += SCISSORS;
                    }
                }
                "A" if player_two == "Y" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += PAPER + WIN;
                    } else {
                        acc += ROCK + DRAW;
                    }
                }
                "A" if player_two == "Z" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += SCISSORS;
                    } else {
                        acc += PAPER + WIN;
                    }
                }
                "B" if player_two == "X" => acc += ROCK,
                "B" if player_two == "Y" => acc += PAPER + DRAW,
                "B" if player_two == "Z" => acc += SCISSORS + WIN,
                "C" if player_two == "X" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += ROCK + WIN
                    } else {
                        acc += PAPER
                    }
                }
                "C" if player_two == "Y" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += PAPER
                    } else {
                        acc += SCISSORS + DRAW
                    }
                }
                "C" if player_two == "Z" => {
                    if cheat_mode == CheatMode::Selection {
                        acc += SCISSORS + DRAW
                    } else {
                        acc += ROCK + WIN
                    }
                }
                _ => (),
            };

            acc
        })
    }
}

#[cfg(test)]
mod tests {

    use super::{CheatMode, DayTwo};

    #[test]
    fn calculates_score_selection_cheatmode() {
        let input: Vec<String> = vec![
            String::from("A Y"), // 4
            String::from("B X"), // 5
            String::from("C Z"), // 6
        ];

        let score = DayTwo::calc_total_score(&input, CheatMode::Selection);
        println!("{score}");
        assert!(score == 15);
    }

    #[test]
    fn calculates_score_outcome_cheatmode() {
        let input: Vec<String> = vec![
            String::from("A Y"), // 4
            String::from("B X"), // 1
            String::from("C Z"), // 7
        ];

        let score = DayTwo::calc_total_score(&input, CheatMode::Outcome);
        assert!(score == 12);
    }
}
