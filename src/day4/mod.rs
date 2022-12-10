use std::collections::HashSet;

pub fn run(input: String) -> (String, String) {
    let mut subset_count = 0;
    let mut intersection_count = 0;
    for line in input.lines() {
        let (first, second) = line.split_once(',').unwrap();
        let first_range = set_from_range_str(first);
        let second_range = set_from_range_str(second);
        if first_range.is_subset(&second_range) || second_range.is_subset(&first_range) {
            subset_count += 1;
        }
        if first_range.intersection(&second_range).count() > 0 {
            intersection_count += 1;
        }
    }
    (subset_count.to_string(), intersection_count.to_string())
}

fn set_from_range_str(str: &str) -> HashSet<u32> {
    let (start, end) = str.split_once('-').unwrap();
    let start: u32 = start.parse().unwrap();
    let end: u32 = end.parse().unwrap();
    (start..=end).collect()
}
