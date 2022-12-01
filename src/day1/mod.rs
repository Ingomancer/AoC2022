use std::{fs::File, io::Read, path::Path};

pub fn run(path: &Path) {
    let s = read_file(path);
    let mut vec: Vec<u32> = Vec::new();
    sum_calories_per_elf(&mut vec, s);
    let mut highest_calories = sum_n_highest(&vec, 1);
    println!("{}", highest_calories);

    highest_calories = sum_n_highest(&vec, 3);
    println!("{}", highest_calories);
}

fn sum_n_highest(vec: &Vec<u32>, n: u32) -> u32 {
    let mut vec = vec.clone();
    let mut sum_cals = 0;

    for _ in 0..n {
        let highest_calories = *vec.iter().max().unwrap();
        let index = vec.iter().position(|&r| r == highest_calories).unwrap();
        vec.remove(index);
        sum_cals += highest_calories;
    }
    sum_cals
}

fn sum_calories_per_elf(vec: &mut Vec<u32>, s: String) {
    vec.push(0);
    for line in s.lines() {
        if line.is_empty() {
            vec.push(0);
        } else {
            let len = vec.len() - 1;
            vec[len] = vec.last().unwrap() + line.parse::<u32>().unwrap();
        }
    }
}

fn read_file(path: &Path) -> String {
    let mut input = File::open(path).expect("No input found");
    let mut s = String::new();
    input.read_to_string(&mut s).expect("Couldn't read input");
    s
}
