use crate::prelude::*;

type Stack = Vec<char>;

pub fn calculate_part1() -> Result<String>{
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Determine number of stacks (and interpret first line while we have it)
    let mut stacks: Vec<Stack>;
    if let Some(line_or_err) = lines.next() {
        let line = line_or_err?;
        stacks = parse_first_line(line);
    } else {
        return Err( anyhow!("Empty file?") )
    }

    // Initialise stacks.
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {break} // delimiter between stack and instructions
        parse_stack(line, &mut stacks);
    }

    // Parse and execute instructions.
    for line_or_err in lines {
        let line = line_or_err?;
        if line.is_empty() {continue}

        let instr = parse_instruction(line)?;
        perform_instruction(&mut stacks, instr);
    }

    // Get the top element from each stack.
    let mut tops = String::new();
    for stack in stacks.iter_mut() {
        if let Some(c) = stack.pop() {
            tops.push(c);
        }
    }
    Ok(tops)
}

/// Parse the first line of the stack to initialise, before doing parsing as normal
fn parse_first_line(line: String) -> Vec<Stack> {
    let len = (line.len()+1) / 4;
    let mut stacks: Vec<Stack> = vec![];
    for _ in 0..len {
        stacks.push(vec![])
    }
    parse_stack(line, &mut stacks);
    stacks
}

/// Parse the stack string and add containers to stacks.
fn parse_stack(line: String, stacks: &mut [Stack]){
    const VALID_FIRST_CHARS: [char;2] = [' ', '['];
    let second_char = line.chars().nth(1).expect("Empty instruction!");
    if second_char == '1' {
        // This is the line after the stack and before instructions, containing the stack numbers 1  2  3  4  ...
        // So do nothing.
        return;
    }
    
    // Iterate over each group of four characters, e.g. "[N] ".
    for (n, container) in line.as_bytes().chunks(4).enumerate() {
        let container_contents = container[1] as char;
        // If the container exists, add it to the stack in the proper place.
        if container_contents != ' ' {
            stacks[n].insert(0, container_contents);
        }
    }

}

struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

fn parse_instruction(line: String) -> Result<Instruction> {
    let words: Vec<&str> = line.split_whitespace().collect();

    Ok(
        Instruction {
            count: words[1].parse()?, 
            source: words[3].parse::<usize>()? - 1, // one-indexed
            dest: words[5].parse::<usize>()? - 1,
        }
    )
}

fn perform_instruction(stacks: &mut [Stack], instr: Instruction) {
    for _ in 0..instr.count {
        let container = stacks[instr.source].pop().expect("Tried to pop empty stack!");
        stacks[instr.dest].push(container);
    }
}

/* Part 2 begins here */

// Identical to above, except use perform_instruction_9001 instead.
pub fn calculate_part2() -> Result<String>{
    let file = File::open("input/day5.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    // Determine number of stacks (also interpret first line)
    let mut stacks: Vec<Stack>;
    if let Some(line_or_err) = lines.next() {
        let line = line_or_err?;
        stacks = parse_first_line(line);
    } else {
        return Err( anyhow!("Empty file?") )
    }

    // Initialise stacks.
    loop {
        let line = lines.next().unwrap()?;
        if line.is_empty() {break} // delimiter between stack and instructions
        parse_stack(line, &mut stacks);
    }

    // Parse and execute instructions.
    for line_or_err in lines {
        let line = line_or_err?;
        if line.is_empty() {continue}

        let instr = parse_instruction(line)?;
        perform_instruction_9001(&mut stacks, instr);
    }

    // Get the top element from each stack.
    let mut tops = String::new();
    for stack in stacks.iter_mut() {
        if let Some(c) = stack.pop() {
            tops.push(c);
        }
    }
    Ok(tops)
}

/// Move crates from source to dest while maintaining their internal order (i.e. not like a stack)
fn perform_instruction_9001(stacks: &mut [Stack], instr: Instruction) {
    let start_pos = (stacks[instr.source].len() - 1) - (instr.count - 1).max(0);
    let containers: Vec<char> = stacks[instr.source].drain(start_pos..).collect();
    for container in containers {
        stacks[instr.dest].push(container);
    }
}