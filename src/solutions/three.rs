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

const LC_A: u8 = "a".as_bytes()[0];
const UC_A: u8 = "A".as_bytes()[0];

const LOW_BASE: u32 = 1;
const HI_BASE: u32 = 27;

impl DayThree {
    // total priority for shared values
    pub fn calc_priority(input: &str) -> u32 {
        input.split("\n").fold(0, |mut acc, sack| {
            let mid = sack.len() / 2;

            let (left, right) = sack.split_at(mid);

            let shared = Self::find_shared_value(left, right, None).unwrap();

            acc += Self::get_priority(shared);
            acc
        })
    }

    pub fn calc_badge_priority(input: &str) -> u32 {
        let bags = input.split("\n").collect::<Vec<&str>>();

        bags.as_slice().chunks(3).fold(0, |mut acc, group| {
            let shared = Self::find_shared_value(group[0], group[1], Some(group[2]));

            if shared.is_some() {
                acc += Self::get_priority(shared.unwrap());
            }

            acc
        })
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

        Some(*shared.get(0).unwrap())
    }

    fn get_priority(value: u8) -> u32 {
        // value is lowercase
        if &value >= &LC_A {
            let diff = value - LC_A;
            return LOW_BASE + diff as u32;
        } else {
            let diff = value - UC_A;
            return HI_BASE + diff as u32;
        }
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
