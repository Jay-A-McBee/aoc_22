use super::solve::Solution;
use regex::Regex;
use std::path::Path;
pub struct DayFour {}

impl Solution for DayFour {
    type Ret = u32;
    type Converted = Vec<[u32; 4]>;

    fn solve() -> Self::Ret {
        let input =
            Self::get_input(&Path::new("static/input_four.txt")).expect("Failed to get input");

        let converted = Self::convert(&input);

        Self::calc_inside_pairs(converted)
    }

    fn convert(input: &str) -> Self::Converted {
        let matcher: Regex =
            Regex::new(r"(?P<p1>\d{1,2})-(?P<p2>\d{1,2}),(?P<p3>\d{1,2})-(?P<p4>\d{1,2})").unwrap();

        input
            .split("\n")
            .map(|line| {
                let matches = matcher.captures(line).unwrap();

                let p1 = &matches["p1"].parse::<u32>().unwrap();
                let p2 = &matches["p2"].parse::<u32>().unwrap();
                let p3 = &matches["p3"].parse::<u32>().unwrap();
                let p4 = &matches["p4"].parse::<u32>().unwrap();

                [*p1, *p2, *p3, *p4]
            })
            .collect::<Vec<[u32; 4]>>()
    }
}

impl DayFour {
    pub fn calc_inside_pairs(groups: Vec<[u32; 4]>) -> u32 {
        groups.iter().fold(0, |mut acc, g| {
            let range_1 = g[0]..=g[1];
            let range_2 = g[2]..=g[3];

            let inside_pair_a = range_1.contains(&g[2]) && range_1.contains(&g[3]);
            let inside_pair_b = range_2.contains(&g[0]) && range_2.contains(&g[1]);

            if inside_pair_a || inside_pair_b {
                acc += 1
            }

            acc
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::solutions::solve::Solution;

    use super::DayFour;

    #[test]
    fn test() {
        let converted = DayFour::convert("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8");

        let answer = DayFour::calc_inside_pairs(converted);

        assert!(answer == 2);
    }
}
