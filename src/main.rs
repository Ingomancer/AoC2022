use std::{error::Error, fs, path::Path};

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main() -> Result<(), Box<dyn Error>> {
    let day: u32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()?;
    let path = format!("src/day{}/input", day);
    let input = fs::read_to_string(Path::new(&path))?;
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        3 => day3::run(input),
        4 => day4::run(input),
        5 => day5::run(input),
        _ => println!("Unknown day"),
    }
    Ok(())
}
