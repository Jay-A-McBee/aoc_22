use super::solution::Solution;
use std::path::Path;
pub struct DayTwo {}

impl Solution for DayTwo {
    type Ret = u32;
    type Converted = Vec<String>;

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("../static/input_two.txt"))
            .expect("Failed to get static file");

        let converted = Self::convert(&input);

        DayTwo::calc_total_score(&converted, CheatMode::Selection)
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
                        acc += 4
                    } else {
                        acc += 3
                    }
                }
                "A" if player_two == "Y" => acc += 8,
                "A" if player_two == "Z" => acc += 3,
                "B" if player_two == "X" => acc += 1,
                "B" if player_two == "Y" => acc += 5,
                "B" if player_two == "Z" => acc += 9,
                "C" if player_two == "X" => acc += 7,
                "C" if player_two == "Y" => acc += 2,
                "C" if player_two == "Z" => acc += 6,
                _ => acc += 0,
            };

            acc
        })
    }

    // calculate total score based on player one selection and desired outcome
    pub fn calc_total_score_with_outcome(input: &Vec<String>) -> u32 {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::DayTwo;

    #[test]
    fn calculates_score() {
        let input: Vec<String> = vec![String::from("AY"), String::from("BX"), String::from("CZ")];

        let score = DayTwo::calc_total_score(&input);
        assert!(score == 15);
    }
}
