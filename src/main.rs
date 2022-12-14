use crate::solutions::{
    five::DayFive, four::DayFour, one::DayOne, seven::DaySeven, six::DaySix, solve::Solution,
    three::DayThree, two::DayTwo,
};

pub mod solutions;

extern crate regex;

fn main() {
    println!("{:?}", DaySeven::solve());
}
