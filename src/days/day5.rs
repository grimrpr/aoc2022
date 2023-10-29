use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
};

struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

impl CargoStacks {
    fn parse_cargo<R>(lines_iterator: &mut Lines<R>) -> Result<Self, Box<dyn Error>>
    where
        R: BufRead,
    {
        let (characters, stack_offsets) = lines_iterator
            .by_ref()
            .map_while(|line| match line.ok() {
                Some(line) if !line.is_empty() => Some(line),
                _ => None,
            })
            .fold(
                (Vec::<(usize, char)>::new(), Vec::<usize>::new()),
                |mut acc, line: String| {
                    if line.contains(char::is_uppercase) {
                        acc.0.extend(
                            line.char_indices()
                                .filter(|(_, letter)| letter.is_uppercase()),
                        );
                    } else {
                        acc.1
                            .extend(line.char_indices().filter_map(|elem| match elem {
                                (idx, letter) if letter.is_numeric() => Some(idx),
                                _ => None,
                            }));
                    }
                    acc
                },
            );

        let mut cargo = CargoStacks {
            stacks: vec![Vec::new(); stack_offsets.len()],
        };

        for (idx, character) in characters.iter().rev() {
            let stack_idx = stack_offsets.binary_search(idx).unwrap();
            cargo.stacks[stack_idx].push(*character);
        }

        Ok(cargo)
    }

    fn move_cargo<R>(&mut self, lines_iterator: &mut Lines<R>) -> Result<(), Box<dyn Error>>
    where
        R: BufRead,
    {
        for mov_op in lines_iterator.by_ref().filter_map(|line| {
            let parse_results = line
                .unwrap()
                .split(' ')
                .filter_map(|word_str| word_str.parse().ok())
                .take(3)
                .collect::<Vec<usize>>();
            if parse_results.len() == 3 {
                Some((parse_results[0], parse_results[1] - 1, parse_results[2] - 1))
            } else {
                None
            }
        }) {
            let range = (self.stacks[mov_op.1].len() - mov_op.0)..(self.stacks[mov_op.1].len());
            let split_idx = (mov_op.1 + mov_op.2) / 2 + 1;
            let (stacks_first, stacks_second) = self.stacks.split_at_mut(split_idx);
            let stack_src;
            let stack_dst;
            if mov_op.1 < mov_op.2 {
                stack_src = &mut (stacks_first[mov_op.1]);
                stack_dst = &mut (stacks_second[mov_op.2 - split_idx]);
            } else {
                stack_src = &mut (stacks_second[mov_op.1 - split_idx]);
                stack_dst = &mut (stacks_first[mov_op.2]);
            }
            stack_dst.extend(stack_src.drain(range).rev());
        }
        Ok(())
    }

    fn top_crates_str(&self) -> String {
        self.stacks.iter().filter_map(|s| s.last()).cloned().collect()
    }
}

pub fn print_answer() {
    let mut line_iterator = BufReader::new(File::open("data/input_day5").unwrap()).lines();
    let mut cargo = CargoStacks::parse_cargo(&mut line_iterator).expect("Failed to parse cargo");
    println!("parsed top crates: {}",cargo.top_crates_str());
    cargo
        .move_cargo(&mut line_iterator)
        .expect("Failed to parse cargo");
    println!("result top crates: {}",cargo.top_crates_str());
}
