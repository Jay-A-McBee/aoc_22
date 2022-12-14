use super::solve::Solution;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::path::Path;

static DISK_SPACE: i32 = 70000000;
static REQUIRED_DISK_SPACE: i32 = 30000000;

pub struct DaySeven {}

impl Solution for DaySeven {
    type Ret = (i32, i32);
    type Converted = ();

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("static/input_seven.txt"))
            .expect("Failed to get static file");

        let flattened = DaySeven::flatten(&input);

        (
            DaySeven::calc_total_size_under_limit(&flattened, 100000),
            DaySeven::find_smallest_dir_over_limit(&flattened),
        )
    }

    fn convert(_input: &str) -> Self::Converted {
        ()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Node<'nd> {
    File(i32),
    Dir(&'nd str),
}

#[derive(Debug)]
pub struct Directory<'nd> {
    name: &'nd str,
    contents: Vec<Node<'nd>>,
}

impl<'nd> Directory<'nd> {
    pub fn new(name: &'nd str, contents: Vec<Node<'nd>>) -> Self {
        Self { name, contents }
    }
}

impl DaySeven {
    pub fn flatten(input: &str) -> Vec<Directory> {
        let mut node_stack: Vec<Directory> = vec![];
        let mut current_dir_name: &str = "";
        let mut current_dir_contents: Vec<Node> = vec![];

        let mut lines = input.split("\n").peekable();

        while let Some(line) = lines.next() {
            if line.starts_with("$ cd") {
                if let [_, _, dir_or_dots] = line.split_whitespace().collect::<Vec<&str>>()[0..=2] {
                    let current_node_empty = current_dir_name.is_empty();

                    match dir_or_dots {
                        ".." if !current_node_empty => {
                            let node =
                                Directory::new(current_dir_name, current_dir_contents.clone());
                            node_stack.push(node);
                            current_dir_name = "";
                            current_dir_contents.clear();
                        }
                        _ if !current_node_empty => {
                            let node =
                                Directory::new(current_dir_name, current_dir_contents.clone());
                            node_stack.push(node);
                            current_dir_name = dir_or_dots;
                            current_dir_contents.clear();
                        }
                        _ => {
                            current_dir_name = dir_or_dots;
                            current_dir_contents.clear();
                        }
                    }
                }
            } else if !line.starts_with("$ ls") {
                if let [file_size_or_dir, name] =
                    line.split_whitespace().collect::<Vec<&str>>()[0..=1]
                {
                    match file_size_or_dir {
                        "dir" => current_dir_contents.push(Node::Dir(name)),
                        _ => current_dir_contents
                            .push(Node::File(file_size_or_dir.parse::<i32>().unwrap())),
                    }
                }
            }

            if lines.peek().is_none() {
                // capture the final node
                let node = Directory::new(current_dir_name, current_dir_contents.clone());
                node_stack.push(node);
            }
        }

        node_stack
    }

    fn find_smallest_dir_over_limit(node_stack: &Vec<Directory>) -> i32 {
        let dir_sizes = Self::get_dir_sizes(&node_stack);
        let space_to_free: i32 = REQUIRED_DISK_SPACE - (DISK_SPACE - dir_sizes.last().unwrap());

        dir_sizes.iter().fold(DISK_SPACE, |mut smallest, size| {
            if size >= &space_to_free && size < &smallest {
                smallest = *size;
            }

            smallest
        })
    }

    fn calc_total_size_under_limit(node_stack: &Vec<Directory>, limit: i32) -> i32 {
        let all_dirs = Self::get_dir_sizes(&node_stack);
        all_dirs.iter().filter(|&size| size <= &limit).sum()
    }

    fn get_dir_sizes(node_stack: &Vec<Directory>) -> Vec<i32> {
        // map of queues
        let mut size_map: HashMap<&str, VecDeque<i32>> = HashMap::new();

        let mut all_dirs: Vec<i32> = Vec::with_capacity(node_stack.len());

        let mut iter = node_stack.iter().rev();

        while let Some(Directory { name, contents }) = iter.next() {
            let total: i32 = contents
                .iter()
                .map(|c| match c {
                    Node::File(size) => *size,
                    Node::Dir(n) => {
                        let dir_stack = size_map.get_mut(n).unwrap();
                        let dir = dir_stack.pop_front().unwrap();
                        dir
                    }
                })
                .sum();

            all_dirs.push(total);

            size_map
                .entry(name)
                .and_modify(|v| v.push_back(total))
                .or_insert(VecDeque::from([total]));
        }

        all_dirs
    }
}

#[cfg(test)]
mod tests {

    use super::DaySeven;

    static INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn finds_total_under_limit() {
        let node_stack = DaySeven::flatten(&INPUT);
        let count = DaySeven::calc_total_size_under_limit(&node_stack, 100000);
        assert!(count == 95437);
    }

    #[test]
    fn finds_smallest_dir_over_limit() {
        let node_stack = DaySeven::flatten(&INPUT);
        let space = DaySeven::find_smallest_dir_over_limit(&node_stack);

        assert!(space == 24933642);
    }
}
