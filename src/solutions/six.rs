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
            Self::find_unique_idx(&input, 3),
            Self::find_unique_idx(&input, 13),
        )
    }

    fn convert(_input: &str) -> Self::Converted {
        ()
    }
}

impl DaySix {
    pub fn find_unique_idx(signals: &str, window_size: usize) -> Option<usize> {
        let mut window = VecDeque::with_capacity(window_size);
        // use a set to limit the amount of times we run 'position' iterator
        let mut set = HashSet::new();
        let mut iter = signals.chars().enumerate();

        while let Some((idx, ch)) = iter.next() {
            let len = window.len();

            if set.contains(&ch) {
                // window contains char
                // clear out all values from window and set that were inserted before
                // the matching instance of this char
                window = Self::clear_window_and_set(&mut set, &mut window, ch);
                window.push_back(ch);
                set.insert(ch);
            } else if len < window_size {
                window.push_back(ch);
                set.insert(ch);
            } else {
                return Some(idx + 1);
            }
        }
        None
    }

    pub fn clear_window_and_set(
        set: &mut HashSet<char>,
        queue: &mut VecDeque<char>,
        ch: char,
    ) -> VecDeque<char> {
        let pos = queue.iter().position(|v| v == &ch).unwrap();

        let keep = queue.split_off(pos + 1);

        // queue is now the chars that were inserted before and up to
        // the matching char. We want to remove all of these from the set.
        while let Some(c) = queue.pop_front() {
            set.remove(&c);
        }

        keep
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
                .all(|(case, expected)| { DaySix::find_unique_idx(case, 3) == *expected })
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
                .all(|(case, expected)| { DaySix::find_unique_idx(case, 13) == *expected })
                == true
        )
    }
}
