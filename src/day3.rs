use crate::prelude::*;

// Assumes input is only ASCII upper/lowercase.
pub fn calculate_part1() -> Result<usize>{
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);
    let mut priority_sum: usize = 0;

    for line in reader.lines() {
        let line = line?;
        if line.is_empty() {continue}

        let mut bitmasks: [u64; 2] = [0; 2];
        let (comp1, comp2) = line.split_at(line.len()/2);

        // For each half, loop through the characters in the string. Convert each to a priority, 
        // then encode in a bitmask by putting a 1 in that position
        for (bitmask, compartment) in bitmasks.iter_mut().zip([comp1, comp2]) {
            for char in compartment.as_bytes() {
                let priority = to_priority(char);
                *bitmask |= 1<<priority;
            }
        }

        // Items common to both elements survive a bitwise AND.
        let common_items_bitmask = bitmasks[0] & bitmasks[1];

        // Assume there is only one shared item => only one non-zero bit => power of two => priority of item is log2.
        let common_item_priority = common_items_bitmask.ilog2() as usize;
        priority_sum += common_item_priority;
    }
    Ok(priority_sum)
}

// Assumes input is only ASCII upper/lowercase.
fn to_priority(ascii: &u8) -> u8 {
    // Some good ol' magic numbers. Convert lowercase ascii from 97+ to 1+, and uppercase from 65+ to 27+
    if ascii <= &90 {ascii - 38} else {ascii - 96}
}

/********** Part 2 **********/

// Very similar to above, but we loop over chunks of GROUP_SIZE.
// Assumes input is a multiple of three lines.
const GROUP_SIZE: usize = 3;
pub fn calculate_part2() -> Result<usize>{
    let file = File::open("input/day3.txt")?;
    let reader = BufReader::new(file);
    let mut priority_sum: usize = 0;

    let lines = reader.lines()
        .map_while(Result::ok)
        .collect::<Vec<String>>();

    for group in lines.chunks(GROUP_SIZE) {
        let mut bitmasks: [u64; GROUP_SIZE] = [0; GROUP_SIZE];
        for (bitmask, rucksack) in bitmasks.iter_mut().zip(group){
            *bitmask = encode_rucksack(rucksack);
        }

        // Items common to all elements survive a bitwise AND.
        let common_items_bitmask = bitmasks.into_iter()
            .reduce(|msk1, msk2| msk1 & msk2)
            .ok_or(anyhow!("Reducing failed!"))?;

        // Assume there is only one shared item => only one non-zero bit => power of two => priority of item is log2.
        let common_item_priority = common_items_bitmask.ilog2() as usize;
        priority_sum += common_item_priority;
    }
    Ok(priority_sum)
}

/// Encode a rucksack into a bitmask where each index of the mask is whether an element is present.
fn encode_rucksack(rucksack: &str) -> u64 {
    let mut bitmask: u64 = 0;
    for char in rucksack.as_bytes() {
        let priority = to_priority(char);
        bitmask |= 1<<priority;
    }
    bitmask
}