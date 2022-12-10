use super::solve::Solution;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct DayFive {
    stacks: HashMap<usize, Stack>,
}

static mut FINAL_IDX: usize = 0;

impl Solution for DayFive {
    type Ret = (String, String);
    type Converted = (Vec<Vec<char>>, Vec<Vec<usize>>);

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("static/input_five.txt"))
            .expect("Failed to get static file");

        let (stack, instructions) = Self::convert(&input);

        let mut preserved_stacks = DayFive::fill_stacks(&stack);
        let mut inverted_stacks = DayFive::fill_stacks(&stack);

        (
            preserved_stacks.process_instructions(&instructions, Order::Preserve),
            inverted_stacks.process_instructions(&instructions, Order::Invert),
        )
    }

    fn convert(input: &str) -> Self::Converted {
        let stacks_instructions = input.split("\n\n").collect::<Vec<&str>>();

        let stacks = stacks_instructions.get(0).unwrap();
        let instructions = stacks_instructions.get(1).unwrap();

        let mut space_count = 0;

        let stack_iter = stacks.split("\n");

        let clean_stacks = stack_iter
            .filter_map(|line| {
                let len = line.len();

                let mut peek_ch = line.chars().peekable();
                let mut mapped: Vec<char> = Vec::with_capacity(len);
                let mut finished = false;

                while let Some(ch) = peek_ch.next() {
                    match ch {
                        '1' => {
                            // we hit the final line
                            finished = true;
                            break;
                        }
                        ' ' if peek_ch.peek().is_some() && peek_ch.peek().unwrap() == &' ' => {
                            // we're at the final space in an empty column
                            // scale these four chars down to a single char
                            // and reset the space count
                            if space_count == 3 {
                                space_count = 0;
                                mapped.push('*');
                            } else {
                                space_count += 1;
                            }
                        }
                        ' ' if peek_ch.peek().is_some() && peek_ch.peek().unwrap() == &'[' => {
                            // line started with an empty column
                            if space_count == 3 {
                                mapped.push('*');
                                space_count = 0;
                            }
                        }
                        ' ' if peek_ch.peek().is_none() => {
                            // line ended with an empty column
                            if space_count == 3 {
                                mapped.push('*');
                                space_count = 0;
                            }
                        }
                        // we only care about consecutive white space
                        ' ' | '[' | ']' => (),
                        _ => mapped.push(ch),
                    }
                }

                if !finished {
                    unsafe {
                        FINAL_IDX = if FINAL_IDX == 0 {
                            mapped.len()
                        } else {
                            FINAL_IDX
                        }
                    }
                    return Some(mapped);
                }

                None
            })
            .collect::<Vec<Vec<char>>>();

        // parse instructions into [[num, num, num]]
        let clean_instructions: Vec<Vec<usize>> = instructions
            .split("\n")
            .map(|instr| {
                instr
                    .split_whitespace()
                    .filter_map(|word| {
                        let ch = word.chars().nth(0).unwrap();
                        if ch.is_digit(10) {
                            return usize::try_from(word.parse::<u32>().unwrap()).ok();
                        }

                        None
                    })
                    .collect()
            })
            .collect::<Vec<Vec<usize>>>();

        (clean_stacks, clean_instructions)
    }
}

#[derive(Debug)]
struct Stack {
    stack: Vec<char>,
}

// Basic stack struct
impl Stack {
    pub fn insert(&mut self, ch: char) {
        if ch != '*' {
            self.stack.push(ch);
        }
    }

    pub fn insert_mult(&mut self, chs: Vec<char>) {
        self.stack.extend(chs.iter());
    }

    pub fn remove(&mut self) -> Option<char> {
        self.stack.pop()
    }
}

#[derive(PartialEq)]
pub enum Order {
    Preserve,
    Invert,
}

impl DayFive {
    pub fn fill_stacks(stacks: &Vec<Vec<char>>) -> Self {
        let mut instance = Self {
            stacks: HashMap::from([
                (1_usize, Stack { stack: Vec::new() }),
                (2_usize, Stack { stack: Vec::new() }),
                (3_usize, Stack { stack: Vec::new() }),
                (4_usize, Stack { stack: Vec::new() }),
                (5_usize, Stack { stack: Vec::new() }),
                (6_usize, Stack { stack: Vec::new() }),
                (7_usize, Stack { stack: Vec::new() }),
                (8_usize, Stack { stack: Vec::new() }),
                (9_usize, Stack { stack: Vec::new() }),
            ]),
        };

        stacks.into_iter().rev().for_each(|vals| {
            vals.into_iter().enumerate().for_each(|(idx, v)| {
                let idx = &idx + &1_usize;
                let stack = instance.stacks.get_mut(&idx).unwrap();
                stack.insert(*v);
            })
        });

        instance
    }

    pub fn process_instructions(
        &mut self,
        instructions: &Vec<Vec<usize>>,
        ordering: Order,
    ) -> String {
        instructions.iter().for_each(|i| {
            let mut insert_items = vec![];
            let mut to_idx = &1_usize;

            if let [count, from, to] = i.as_slice() {
                to_idx = to;
                let exit_stack = self.stacks.get_mut(from).unwrap();
                let len = exit_stack.stack.len();

                let range = if len > *count { len - *count.. } else { 0.. };

                insert_items = if ordering == Order::Preserve {
                    exit_stack.stack.drain(range).rev().collect()
                } else {
                    exit_stack.stack.drain(range).collect()
                };
            }

            let enter_stack = self.stacks.get_mut(&to_idx).unwrap();
            enter_stack.insert_mult(insert_items);
        });

        unsafe {
            (1..=FINAL_IDX).fold(String::new(), |mut acc, idx| {
                let stack = self.stacks.get_mut(&idx).unwrap();
                if let Some(ch) = stack.remove() {
                    acc.push(ch);
                }
                acc
            })
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const DATA: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn moves_and_preserves_order() {
        let (stacks, instructions) = DayFive::convert(DATA);

        let mut instance = DayFive::fill_stacks(&stacks);

        let answer = instance.process_instructions(&instructions, Order::Preserve);

        assert!(answer == String::from("CMZ"))
    }

    #[test]
    fn moves_and_inverts_order() {
        let (stacks, instructions) = DayFive::convert(DATA);

        let mut instance = DayFive::fill_stacks(&stacks);

        let answer = instance.process_instructions(&instructions, Order::Invert);

        assert!(answer == String::from("MCD"))
    }
}
