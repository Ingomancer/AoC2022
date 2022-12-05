use std::collections::{HashMap, VecDeque};

use itertools::Itertools;

pub fn run(input: String) {
    let mut instructions = false;
    
    let mut stacks = HashMap::new();
    let mut stacks2;
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
                    stacks.entry(count).and_modify(|v: &mut VecDeque<char>| v.push_front(letter)).or_insert(VecDeque::from([letter]));
                }
                count += 1;
            }
        } else {
            if line.len() == 0 {
                continue
            }
            let line = line.replace("move", "").replace("from", "").replace("to","");
            let (count, start, end) = line.split_whitespace().next_tuple().unwrap();
            let count: u32 = count.parse().unwrap();
            let start: usize = start.parse().unwrap();
            let end: usize = end.parse().unwrap();
            for _ in 0..count {
                let moved_box = stacks.get_mut(&start).unwrap().pop_back().unwrap();
                stacks.get_mut(&end).unwrap().push_back(moved_box);
            }
        }
    }
    for index in 1..stacks.len()+1{
        print!("{}", stacks.get(&index).unwrap().back().unwrap());
    }

}

// move 3 from 9 to 7