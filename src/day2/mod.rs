use std::{fmt::Error, collections::HashMap};


pub fn run(input: String){
    let mut score1 = 0;
    let mut score2 = 0;
    let mut beats = HashMap::new();
    beats.insert("A", "Y");
    beats.insert("B", "Z");
    beats.insert("C", "X");
    beats.insert("X", "B");
    beats.insert("Y", "C");
    beats.insert("Z", "A");
    let choices = vec!["X", "Y", "Z"];
    for line in input.lines() {
        let (opp, you) = line.split_once(' ').unwrap();

        score1 += play_rps(opp, you, &beats) + score_shape(you);
        let your_move;
        let expected_outcome;
        if you == "X" {
            expected_outcome = 0;
            your_move = find_move_with_outcome(&choices, opp, &beats, expected_outcome);
        } else if you == "Y" {
            expected_outcome = 3;
            your_move = find_move_with_outcome(&choices, opp, &beats, expected_outcome);
        } else {
            expected_outcome = 6;
            your_move = find_move_with_outcome(&choices, opp, &beats, expected_outcome);
        }
        score2 += play_rps(opp, your_move, &beats) + score_shape(your_move);
    }
    println!("{}", score1);
    println!("{}", score2);
}

fn find_move_with_outcome<'a>(choices: &'a Vec<&str>, opp: &str, beats: &HashMap<&str, &str>, new_score: i32) -> &'a str{
    for choice in choices {
        if play_rps(opp, choice, beats) == new_score {
            return choice
        }
    }
    ""
}

fn play_rps(opp: &str, you: &str, beats: &HashMap<&str, &str>) -> i32 {
    if beats[opp] == you {
        6
    } else if beats[you] == opp {
        0
    } else {
        3
    }
}

fn score_shape(shape: &str) -> i32 {
    if shape == "X" {
        1
    } else if shape == "Y" {
        2
    } else {
        3
    }
}