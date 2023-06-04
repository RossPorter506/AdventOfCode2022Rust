#![allow(dead_code)]

mod day1; use day1::*;
use anyhow::Result;

/**/
// https://adventofcode.com/2022
/**/

fn main() -> Result<()>{
    //let result = calculate_part1()?;
    let result = calculate_part2::<3>()?;
    println!("{result}");
    Ok(())
}
