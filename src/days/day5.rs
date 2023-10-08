use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

struct CargoStacks {
    stacks: Vec<Vec<char>>,
}

impl CargoStacks {
    fn parse_cargo<R>(reader: R) -> Result<Self, Box<dyn Error>>
    where
        R: BufRead,
    {
        let mut line_iterator = reader.lines();
        let (characters, stack_offsets) = line_iterator
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

        Ok(CargoStacks {
            stacks: vec![vec![]],
        })
    }
}

pub fn print_answer() {
    CargoStacks::parse_cargo(BufReader::new(File::open("data/input_day5").unwrap()))
        .expect("Failed to parse cargo");
}
