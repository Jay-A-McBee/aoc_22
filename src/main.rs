use crate::solutions::{four::DayFour, one::DayOne, solve::Solution, three::DayThree, two::DayTwo};

pub mod solutions;

extern crate regex;

fn main() {
    println!("{:?}", DayFour::solve());
}
