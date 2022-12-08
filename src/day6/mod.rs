use std::collections::HashSet;

pub fn run(input: String) -> (String, String) {
    let mut prev_chars = vec![];
    let mut found = false;
    let mut part1 = String::new();
    let mut part2 = String::new();
    for (i, char) in input.chars().enumerate() {
        prev_chars.push(char);

        if prev_chars.len() >= 4 && !found {
            let prev: HashSet<char> = HashSet::from_iter(prev_chars.iter().rev().take(4).cloned());
            if prev.len() == 4 {
                part1 = (i + 1).to_string();
                found = true;
            }
        }
        if prev_chars.len() >= 14 {
            let prev: HashSet<char> = HashSet::from_iter(prev_chars.iter().rev().take(14).cloned());
            if prev.len() == 14 {
                part2 = (i + 1).to_string();
                break;
            }
        }
    }
    (part1, part2)
}
