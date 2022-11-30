use std::path::Path;

mod day1;
mod day2;

fn main() {
    let day: u32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()
        .expect("Day must be a number");
    let path = format!("src/day{}/input", day);
    let input = &Path::new(&path);
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        _ => println!("Unkown day"),
    }
}
