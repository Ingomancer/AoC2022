use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn run(input: String) -> (String, String) {
    let mut instructions = false;

    let mut stacks = HashMap::new();
    let mut stacks2 = HashMap::new();
    for line in input.lines() {
        if !instructions {
            let mut count: usize = 1;
            for mut chunk in line.chars().chunks(4).into_iter() {
                chunk.next();
                let letter = chunk.next().unwrap();
                if letter == '1' {
                    instructions = true;
                    stacks2 = stacks.clone();
                    break;
                } else if letter != ' ' {
                    stacks
                        .entry(count)
                        .and_modify(|v: &mut VecDeque<char>| v.push_front(letter))
                        .or_insert_with(|| VecDeque::from([letter]));
                }
                count += 1;
            }
        } else {
            if line.is_empty() {
                continue;
            }
            let line = line
                .replace("move", "")
                .replace("from", "")
                .replace("to", "");
            let (count, start, end) = line.split_whitespace().next_tuple().unwrap();
            let count: u32 = count.parse().unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            let mut p2boxes = VecDeque::<char>::new();
            for _ in 0..count {
                let moved_box = stacks.get_mut(&start).unwrap().pop_back().unwrap();
                stacks.get_mut(&end).unwrap().push_back(moved_box);
                let moved_box = stacks2.get_mut(&start).unwrap().pop_back().unwrap();
                p2boxes.push_front(moved_box);
            }
            stacks2.get_mut(&end).unwrap().append(&mut p2boxes);
        }
    }
    let mut part1 = String::new();
    for index in 1..stacks.len() + 1 {
        part1.push(*stacks.get(&index).unwrap().back().unwrap());
    }

    let mut part2 = String::new();
    for index in 1..stacks2.len() + 1 {
        part2.push(*stacks2.get(&index).unwrap().back().unwrap());
    }
    (part1, part2)
}

// move 3 from 9 to 7
