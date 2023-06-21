use std::collections::VecDeque;

use crate::prelude::*;
const START_OF_PACKET_LEN: usize = 4;
const START_OF_MESSAGE_LEN: usize = 14;

pub fn calculate_part1() -> Result<usize>{
    let contents = std::fs::read_to_string("input/day6.txt")?;
    let mut ringbuf = VecDeque::<char>::with_capacity(START_OF_PACKET_LEN);
    let mut characters = contents.chars();
    let mut count: usize = 0;

    // Fill up ring buf
    for _ in 0..START_OF_PACKET_LEN {
        ringbuf.push_back(characters.next().expect("Stream too short!"));
        count += 1;
    }

    for chr in characters {
        if check_for_unique(&ringbuf, START_OF_PACKET_LEN) {
            println!("{}", ringbuf.iter().collect::<String>());
            return Ok(count)
        }
        ringbuf.push_back(chr);
        ringbuf.pop_front();
        count += 1;
    }
    Err(anyhow!("Start of packet not found"))
}

// Given a buffer of chars, check that all elements are unique.
fn check_for_unique(buf: &VecDeque<char>, size: usize) -> bool {
    // Build up a string and check if each new char is already in the string.
    let mut test_string = String::from(buf[0]);
    for n in 1..size {
        if test_string.contains(buf[n]) { 
            return false;
        } else { 
            test_string.push(buf[n]);
        }
    }
    true
}

/* Part 2 begins here */

// Almost identical, only difference is we use START_OF_MESSAGE_LEN instead of START_OF_PACKET_LEN.
pub fn calculate_part2() -> Result<usize>{
    let contents = std::fs::read_to_string("input/day6.txt")?;
    let mut ringbuf = VecDeque::<char>::with_capacity(START_OF_MESSAGE_LEN);
    let mut characters = contents.chars();
    let mut count: usize = 0;

    // Fill up ring buf
    for _ in 0..START_OF_MESSAGE_LEN {
        ringbuf.push_back(characters.next().expect("Stream too short!"));
        count += 1;
    }

    for chr in characters {
        if check_for_unique(&ringbuf, START_OF_MESSAGE_LEN) {
            println!("{}", ringbuf.iter().collect::<String>());
            return Ok(count)
        }
        ringbuf.push_back(chr);
        ringbuf.pop_front();
        count += 1;
    }
    Err(anyhow!("Start of message not found"))
}