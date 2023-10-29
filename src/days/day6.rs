use std::{
    fs::File,
    io::{BufReader, Read},
};

fn check_disjoint(pattern: &str) -> bool {
    match pattern.as_bytes() {
        [a,..]  => !pattern[1..].as_bytes().contains(a) && check_disjoint(&pattern[1..]),
        _ => true
    }
}

pub fn print_answer() {
    let mut buf_read = BufReader::new(File::open("data/input_day6").unwrap());
    let mut message = "".to_string();
    buf_read.read_to_string(&mut message).expect("Unable to read the file");

    // part 1
    const START_PATTERN_LEN: usize = 4;
    if message.len() < START_PATTERN_LEN {
        println!("String too short {}", message.len());
        return;
    }
    for offset in 0..(message.len() - START_PATTERN_LEN) {
        if check_disjoint(&message[offset..(offset + START_PATTERN_LEN)]) {
            println!("Offset packet: {}", offset + START_PATTERN_LEN);
            break;
        }
    }

    // part 2
    const MESSAGE_PATTERN_LEN: usize = 14;
    if message.len() < MESSAGE_PATTERN_LEN {
        println!("String too short for message {}", message.len());
        return;
    }
    for offset in 0..(message.len() - MESSAGE_PATTERN_LEN) {
        if check_disjoint(&message[offset..(offset + MESSAGE_PATTERN_LEN)]) {
            println!("Offset message: {}", offset + MESSAGE_PATTERN_LEN);
            break;
        }
    }
}
