use super::solve::Solution;
use std::collections::{HashSet, VecDeque};
use std::path::Path;

pub struct DaySix {}

impl Solution for DaySix {
    type Ret = (Option<usize>, Option<usize>);
    type Converted = ();

    fn solve() -> Self::Ret {
        let input =
            Self::get_input(&Path::new("static/input_six.txt")).expect("Failed to get static file");

        (
            Self::find_unique_idx(&input, 4),
            Self::find_unique_idx(&input, 14),
        )
    }

    fn convert(_input: &str) -> Self::Converted {
        ()
    }
}

impl DaySix {
    pub fn find_unique_idx(signals: &str, window_size: usize) -> Option<usize> {
        // queue of chars
        let mut window = VecDeque::with_capacity(window_size);
        // use a set so we don't have to iterate over the window on every char
        let mut set = HashSet::with_capacity(window_size);
        let mut iter = signals.chars().enumerate();

        while let Some((idx, curr)) = iter.next() {
            // pre-insert length
            let prev_len = set.len();

            set.insert(curr);
            window.push_back(curr);

            // post-insert length
            let post_len = set.len();

            // we inserted a duplicate
            if post_len == prev_len {
                while let Some(seen) = window.pop_front() {
                    // Remove values from the set that aren't curr.
                    if seen != curr {
                        set.remove(&seen);
                    } else {
                        // We matched curr. We've removed all the necessary
                        // values from the set and window.
                        break;
                    }
                }
            } else if post_len == window_size {
                return Some(idx + 1);
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {

    use super::DaySix;

    #[test]
    fn finds_first_unique_signal() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(7_usize)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(5_usize)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(6_usize)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(10_usize)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(11_usize)),
        ];

        assert!(
            cases
                .iter()
                .all(|(case, expected)| { DaySix::find_unique_idx(case, 4) == *expected })
                == true
        )
    }

    #[test]
    fn finds_first_unique_msg() {
        let cases = vec![
            ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", Some(19_usize)),
            ("bvwbjplbgvbhsrlpgdmjqwftvncz", Some(23_usize)),
            ("nppdvjthqldpwncqszvftbrmjlhg", Some(23_usize)),
            ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", Some(29_usize)),
            ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", Some(26_usize)),
        ];

        assert!(
            cases
                .iter()
                .all(|(case, expected)| { DaySix::find_unique_idx(case, 14) == *expected })
                == true
        )
    }
}
