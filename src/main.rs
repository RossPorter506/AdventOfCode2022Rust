#![allow(dead_code)]

mod day1;
mod day2;
mod day3; 
mod day4; use day4::*;
mod prelude; use prelude::*;

/**/
// https://adventofcode.com/2022
/**/

fn main() -> Result<()>{
    let result = calculate_part2()?;
    println!("{result}");
    Ok(())
}
