use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use anyhow::{Result,anyhow};

/**/
// https://adventofcode.com/2022/day/1
/**/

// Read in a list of values separated by newlines. Sum until empty line. Return largest.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let mut top_elf_calories = 0;
    let mut elf_calories: usize = 0;

    for line in reader.lines() {
        let line = line?;
        match &line {
            s if s.is_empty() => { // Elf finished. Compare against current highest, and prepare for new elf.
                top_elf_calories = top_elf_calories.max(elf_calories);
                elf_calories = 0;
            },
            _ => elf_calories += line.parse::<usize>()?, // Otherwise just add to current elf.
        };
    }
    Ok(top_elf_calories)
}

// Same as above, but return the sum of the N largest elements.
pub fn calculate_part2<const N: usize>() -> Result<usize>{
    let file = File::open("input/day1.txt")?;
    let reader = BufReader::new(file);

    let mut top_elf_calories = [0usize; N];
    let mut elf_calories: usize = 0;

    for line in reader.lines() {
        let line = line?;
        match &line {
            s if s.is_empty() => {
                let smallest = get_index_of_smallest(&top_elf_calories)?;
                top_elf_calories[smallest] = elf_calories.max(top_elf_calories[smallest]);
                elf_calories = 0;
            },
            _ => elf_calories += line.parse::<usize>()?,
        };
    }
    Ok(top_elf_calories.iter().sum())
}

// Given a slice, return the index of the smallest element.
fn get_index_of_smallest(arr: &[usize]) -> Result<usize> {
    arr.iter()
        .enumerate()
        .min_by(|(_, x), (_, x2)| x.cmp(x2))
        .map(|(i,_)| i)
        .ok_or(anyhow!("No elves!"))
}