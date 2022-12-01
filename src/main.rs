use std::{path::Path, fs::File, io::Read};

mod day1;
mod day2;

fn main() {
    let day: u32 = std::env::args()
        .nth(1)
        .expect("Must supply a day")
        .parse()
        .expect("Day must be a number");
    let path = format!("src/day{}/input", day);
    let input = read_file(&Path::new(&path));
    match day {
        1 => day1::run(input),
        2 => day2::run(input),
        _ => println!("Unknown day"),
    }
}

fn read_file(path: &Path) -> String {
    let mut input = File::open(path).expect("No input found");
    let mut s = String::new();
    input.read_to_string(&mut s).expect("Couldn't read input");
    s
}
