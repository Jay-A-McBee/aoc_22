use crate::solutions::{
    five::DayFive, four::DayFour, one::DayOne, six::DaySix, solve::Solution, three::DayThree,
    two::DayTwo,
};

pub mod solutions;

extern crate regex;

fn main() {
    println!("{:?}", DaySix::solve());
}
