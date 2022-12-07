use super::solve::Solution;
use std::collections::HashSet;
use std::path::Path;

pub struct DayThree {}

impl Solution for DayThree {
    type Ret = (u32, u32);
    type Converted = ();

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("static/input_three.txt"))
            .expect("Failed to get static file");

        (
            DayThree::calc_priority(&input),
            DayThree::calc_badge_priority(&input),
        )
    }

    fn convert(input: &str) -> Self::Converted {
        ()
    }
}

static LC_A: u8 = "a".as_bytes()[0];
static UC_A: u8 = "A".as_bytes()[0];

static LOW_BASE: u32 = 1;
static HI_BASE: u32 = 27;

impl DayThree {
    // total priority for shared values
    pub fn calc_priority(input: &str) -> u32 {
        input
            .split("\n")
            .map(|sack| {
                let end = sack.len();
                let mid = end / 2;

                let shared = Self::find_shared_value(
                    sack.chars().take(mid).collect::<String>().as_str(),
                    sack.chars()
                        .skip(mid)
                        .take(mid)
                        .collect::<String>()
                        .as_str(),
                    None,
                )
                .unwrap();

                Self::get_priority(shared)
            })
            .sum()
    }

    pub fn calc_badge_priority(input: &str) -> u32 {
        let bags = input.split("\n").collect::<Vec<&str>>();

        bags.as_slice()
            .chunks(3)
            .map(|group| {
                let shared = Self::find_shared_value(group[0], group[1], Some(group[2]));

                if shared.is_some() {
                    Self::get_priority(shared.unwrap())
                } else {
                    0
                }
            })
            .sum()
    }

    // finds intersection between up to 3 byte sets
    fn find_shared_value(a: &str, b: &str, c: Option<&str>) -> Option<u8> {
        let set_a: HashSet<u8> = HashSet::from_iter(a.as_bytes().to_owned());
        let set_b: HashSet<u8> = HashSet::from_iter(b.as_bytes().to_owned());

        if let Some(inner) = c {
            return inner
                .as_bytes()
                .iter()
                .find(|&v| set_a.contains(v) && set_b.contains(v))
                .copied();
        }

        let shared: Vec<u8> = set_a.intersection(&set_b).copied().collect();

        Some(shared.get(0).copied().unwrap())
    }

    fn get_priority(value: u8) -> u32 {
        if Self::is_lowercase(&value) {
            let diff = value - LC_A;
            return LOW_BASE + diff as u32;
        } else {
            let diff = value - UC_A;
            return HI_BASE + diff as u32;
        }
    }

    fn is_lowercase(value: &u8) -> bool {
        if value >= &LC_A {
            return true;
        }

        false
    }
}

#[cfg(test)]
mod tests {

    use super::DayThree;

    #[test]
    fn calculates_priority() {
        let value = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        let total = DayThree::calc_priority(value);

        assert!(total == 157)
    }

    #[test]
    fn calculates_badge_priority() {
        let value = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";

        let total = DayThree::calc_badge_priority(value);

        assert!(total == 70)
    }
}
