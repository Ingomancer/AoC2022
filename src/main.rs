use std::{error::Error, fs, path::Path};

mod day1;
mod day2;

fn main() -> Result<(), Box<dyn Error>> {
    let day: u32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()
        .expect("Day must be a number");
    let path = format!("src/day{}/input", day);
    let input = fs::read_to_string(Path::new(&path))?;
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        _ => println!("Unknown day"),
    }
    Ok(())
}
