use crate::solutions::one::DayOne;
use crate::solutions::solve::Solution;
use crate::solutions::three::DayThree;
use crate::solutions::two::DayTwo;

pub mod solutions;

fn main() {
    println!("{:?}", DayThree::solve());
}
