#![allow(dead_code)]


mod day1;
mod day2;
mod day3; 
mod day4; 
mod day5;
mod day6; use day6::*;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2022
/**/

fn main() -> Result<()>{
    let result = calculate_part2()?;
    println!("{result}");
    Ok(())
}
