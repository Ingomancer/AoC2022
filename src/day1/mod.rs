use std::{fs::File, io::Read, path::Path};

pub fn run(path: &Path) {
    let mut input = File::open(path).expect("No input found");
    let mut s = String::new();
    input.read_to_string(&mut s).expect("Couldn't read input");
    for line in s.lines() {
        println!("{}", line)
    }
}
