use super::solution::Solution;
use std::path::Path;
pub struct DayOne {}

impl Solution for DayOne {
    type Ret = (u32, u32);
    type Converted = Vec<u32>;

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("../static/input_one.txt"))
            .expect("Failed to get static file");

        let mut converted = Self::convert(&input);

        DayOne::find_max(&mut converted)
    }

    fn convert(input: &str) -> Self::Converted {
        input
            .split("\n\n")
            .map(|pouch| {
                pouch
                    .split("\n")
                    .map(|cal| cal.parse::<u32>().expect("Failed to parse number"))
                    .sum::<u32>()
            })
            .collect::<Vec<u32>>()
    }
}

impl DayOne {
    // find max value and sum of 3 max values in unsorted array
    pub fn find_max(input: &mut Vec<u32>) -> (u32, u32) {
        input.sort_by(|a, b| b.cmp(a));

        let top_3 = input.iter().take(3).copied().collect::<Vec<u32>>();

        (*top_3.get(0).unwrap(), top_3.iter().sum())
    }
}

#[cfg(test)]
mod tests {

    use super::DayOne;

    #[test]
    fn finds_max() {
        let input: &mut Vec<u32> = &mut vec![1, 9, 6, 7, 5, 8, 4, 3, 2];

        let (max, max_3) = DayOne::find_max(input);
        assert!(max == 9);
        assert!(max_3 == 24);
    }
}
