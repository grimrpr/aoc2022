// mod day1

use std::cmp::max;
use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

pub fn print_answer() {
    println!(
        "{}",
        read_calories(BufReader::new(File::open("../data/input_day1").unwrap())).unwrap()
    );
}

fn read_calories<R>(reader: R) -> Result<i32, Box<dyn Error>>
where
    R: BufRead,
{
    Ok(reader
        .lines()
        .try_fold(
            (0, 0),
            |(acc, max_sum), line| -> Result<(i32, i32), Box<dyn Error>> {
                match line?.trim() {
                    "" => Ok((0, max(acc, max_sum))),
                    l => {
                        let new_acc = acc + i32::from_str(l)?;
                        Ok((new_acc, max(new_acc, max_sum)))
                    }
                }
            },
        )?
        .1)
}
