use std::collections::HashSet;

pub fn run(input: String) {
    let mut prev_chars = vec![];
    let mut found = false;
    for (i, char) in input.chars().enumerate() {
        prev_chars.push(char);

        if prev_chars.len() >= 4 && !found {
            let prev: HashSet<char> = HashSet::from_iter(prev_chars.iter().rev().take(4).cloned());
            if prev.len() == 4 {
                println!("{}", i + 1);
                found = true;
            }
        }
        if prev_chars.len() >= 14 {
            let prev: HashSet<char> = HashSet::from_iter(prev_chars.iter().rev().take(14).cloned());
            if prev.len() == 14 {
                println!("{}", i + 1);
                break;
            }
        }
    }
}
