use crate::prelude::*;
use std::ops::RangeInclusive;

pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);
    let mut count: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {continue}

        let (range1, range2) = parse_line(line)?;

        if subset(range1, range2) {
            count += 1;
        }
    }
    Ok(count)
}

//Assumes input is of the form X-Y,Z-W. Returns the ranges X..=Y and Z..=W.
fn parse_line(line: String) -> Result<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    let mut ranges: Vec<RangeInclusive<u32>> = vec![];
    let range_strings: Vec<&str> = line.split(',').collect();
    
    for range_str in [range_strings[0], range_strings[1]] {
        let values: Vec<&str> = range_str.split('-').collect();
        let start: u32 = values[0].parse()?;
        let end:   u32 = values[1].parse()?;
        ranges.push(start..=end);
    }
    Ok((ranges[0].clone(), ranges[1].clone()))
}

/// Returns true if r1 is completely contained inside r2, or vice versa.
fn subset<T: PartialOrd>(r1: RangeInclusive<T>, r2: RangeInclusive<T>) -> bool {
    (r1.contains(r2.start()) && r1.contains(r2.end())) ||
    (r2.contains(r1.start()) && r2.contains(r1.end()))
}

/* Part 2 starts */

// Almost identical to above, but uses overlap instead of subset.
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day4.txt")?;
    let reader = BufReader::new(file);
    let mut count: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {continue}

        let (range1, range2) = parse_line(line)?;

        if overlap(range1, range2) {
            count += 1;
        }
    }
    Ok(count)
}
/// Returns true if there is any elements that are in both r1 and r2.
fn overlap<T: PartialOrd>(r1: RangeInclusive<T>, r2: RangeInclusive<T>) -> bool {
    (r1.contains(r2.start()) || r1.contains(r2.end())) ||
    (r2.contains(r1.start()) || r2.contains(r1.end()))
}
