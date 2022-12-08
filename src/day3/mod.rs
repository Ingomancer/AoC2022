pub fn run(input: String) -> (String, String) {
    (part_1(&input).to_string(), part_2(&input).to_string())
}

fn part_1(input: &str) -> u32 {
    let mut sum = 0;
    for line in input.lines() {
        let mid = line.len() / 2;
        let first = &line[..mid];
        let second = &line[mid..];
        for c in first.chars() {
            if second.contains(c) {
                sum += priority(c);
                break;
            }
        }
    }
    sum
}

fn part_2(input: &str) -> u32 {
    let mut sum = 0;
    let mut prev1 = "";
    let mut prev2 = "";
    for line in input.lines() {
        if prev1.is_empty() {
            prev1 = line;
        } else if prev2.is_empty() {
            prev2 = prev1;
            prev1 = line;
        } else {
            for c in line.chars() {
                if prev1.contains(c) && prev2.contains(c) {
                    sum += priority(c);
                    prev1 = "";
                    prev2 = "";
                    break;
                }
            }
        }
    }
    sum
}

fn priority(c: char) -> u32 {
    if c.is_lowercase() {
        c as u32 - 96
    } else {
        c as u32 - 38
    }
}
