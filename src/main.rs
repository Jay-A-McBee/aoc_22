use crate::solutions::{one::DayOne, solve::Solution, three::DayThree, two::DayTwo};

pub mod solutions;

fn main() {
    println!("{:?}", DayThree::solve());
}
