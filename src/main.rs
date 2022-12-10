use crate::solutions::{
    five::DayFive, four::DayFour, one::DayOne, solve::Solution, three::DayThree, two::DayTwo,
};

pub mod solutions;

extern crate regex;

fn main() {
    println!("{:?}", DayFive::solve());
}
