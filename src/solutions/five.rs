use super::solve::Solution;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct DayFive {
    stacks: HashMap<usize, Stack>,
}

impl Solution for DayFive {
    type Ret = (String, String);
    type Converted = (Vec<Vec<char>>, Vec<Vec<usize>>, usize);

    fn solve() -> Self::Ret {
        let input = Self::get_input(&Path::new("static/input_five.txt"))
            .expect("Failed to get static file");

        let (stack_lines, instructions, stack_count) = Self::convert(&input);

        let mut inverted_stacks = DayFive::fill_stacks(&stack_lines, &stack_count);
        let mut preserved_stacks = DayFive::fill_stacks(&stack_lines, &stack_count);

        (
            inverted_stacks
                .process_instructions(&instructions, Order::Invert)
                .get_final_value(&stack_count),
            preserved_stacks
                .process_instructions(&instructions, Order::Preserve)
                .get_final_value(&stack_count),
        )
    }

    fn convert(input: &str) -> Self::Converted {
        let stack_instruction_lines = input.split("\n\n").collect::<Vec<&str>>();

        let stack_lines = stack_instruction_lines.get(0).unwrap();
        let instruction_lines = stack_instruction_lines.get(1).unwrap();

        let mut space_count = 0;

        let clean_stack_lines = stack_lines
            .split("\n")
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
                    return Some(mapped);
                }

                None
            })
            .collect::<Vec<Vec<char>>>();

        // parse instructions into [[num, num, num]]
        let clean_instruction_lines: Vec<Vec<usize>> = instruction_lines
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

        let stack_count = clean_stack_lines.get(0).unwrap().len();

        (clean_stack_lines, clean_instruction_lines, stack_count)
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
    pub fn fill_stacks(stack_lines: &Vec<Vec<char>>, stack_count: &usize) -> Self {
        let mut instance = Self {
            stacks: (1..=*stack_count)
                .map(|val| (val, Stack { stack: Vec::new() }))
                .collect::<HashMap<usize, Stack>>(),
        };

        stack_lines.into_iter().rev().for_each(|vals| {
            vals.into_iter().enumerate().for_each(|(idx, v)| {
                let idx = &idx + &1_usize;
                let stack = instance.stacks.get_mut(&idx).unwrap();
                stack.insert(*v);
            })
        });

        instance
    }

    pub fn get_final_value(&mut self, stack_count: &usize) -> String {
        (1..=*stack_count).fold(String::new(), |mut acc, idx| {
            let stack = self.stacks.get_mut(&idx).unwrap();
            if let Some(ch) = stack.remove() {
                acc.push(ch);
            }
            acc
        })
    }

    pub fn process_instructions(
        &mut self,
        instructions: &Vec<Vec<usize>>,
        ordering: Order,
    ) -> &mut Self {
        instructions.iter().for_each(|i| {
            if let [count, from, to] = i.as_slice() {
                let exit_stack = self.stacks.get_mut(from).unwrap();
                let len = exit_stack.stack.len();

                let range = if len > *count { len - *count.. } else { 0.. };

                let insert_items = if ordering == Order::Preserve {
                    exit_stack.stack.drain(range).collect()
                } else {
                    exit_stack.stack.drain(range).rev().collect()
                };

                let enter_stack = self.stacks.get_mut(&to).unwrap();

                enter_stack.insert_mult(insert_items);
            }
        });

        self
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
        let (stacks, instructions, stack_count) = DayFive::convert(DATA);

        let mut instance = DayFive::fill_stacks(&stacks, &stack_count);

        let answer = instance
            .process_instructions(&instructions, Order::Preserve)
            .get_final_value(&stack_count);

        assert!(answer == String::from("MCD"))
    }

    #[test]
    fn moves_and_inverts_order() {
        let (stacks, instructions, stack_count) = DayFive::convert(DATA);

        let mut instance = DayFive::fill_stacks(&stacks, &stack_count);

        let answer = instance
            .process_instructions(&instructions, Order::Invert)
            .get_final_value(&stack_count);

        assert!(answer == String::from("CMZ"))
    }
}
