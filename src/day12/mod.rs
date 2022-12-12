use std::{
    collections::{HashMap},
};

pub fn run(input: String) -> (String, String) {
    let mut grid: Vec<Vec<u32>> = vec![];
    let mut cur_pos = (0, 0);
    let mut goal_pos = (0, 0);
    let mut p2_starts = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut grid_row = vec![];
        for (x, letter) in line.chars().enumerate() {
            if letter == 'S' {
                cur_pos = (y, x);
                grid_row.push('a' as u32);
                p2_starts.push((y, x));
            } else if letter == 'E' {
                goal_pos = (y, x);
                grid_row.push('z' as u32);
            } else if letter == 'a' {
                grid_row.push(letter as u32);
                p2_starts.push((y, x));
            } else {
                grid_row.push(letter as u32);
            }
        }
        grid.push(grid_row);
    }
    let mut visited = HashMap::new();
    let path = find_goal(&grid, cur_pos, goal_pos, &mut visited, 0);
    let mut shortest = u32::MAX;
    for start in p2_starts {
        let path = find_goal(&grid, start, goal_pos, &mut visited, 0);
        if path < shortest {
            shortest = path;
        }
    }
    (
        format!("{path}"),
        format!("{shortest}"),
    )
}

fn find_goal(
    grid: &Vec<Vec<u32>>,
    cur_pos: (usize, usize),
    goal_pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), u32>,
    path_len: u32,
) -> u32 {
    if visited.contains_key(&cur_pos) {
        let len = visited[&cur_pos];
        if path_len >= len {
            return u32::MAX;
        }
    }
    visited.insert(cur_pos, path_len);

    if cur_pos == goal_pos {
        return path_len;
    }

    let mut possible_moves = find_possible_moves(grid, cur_pos);
    if possible_moves.is_empty() {
        return u32::MAX;
    }
    possible_moves.sort_by_key(|a| manhattan(a, &goal_pos));
    let mut min = u32::MAX;
    for new_pos in possible_moves {
        let path = find_goal(grid, new_pos, goal_pos, visited, path_len + 1);
        if path < min {
            min = path;
        }
    }
    visited.insert(cur_pos, path_len);
    min
}

fn manhattan(a: &(usize, usize), b: &(usize, usize)) -> usize {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn find_possible_moves(grid: &Vec<Vec<u32>>, cur_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let neighbours = generate_neighbours(cur_pos);
    neighbours
        .into_iter()
        .filter(|x| {
            x.0 < grid.len()
                && x.1 < grid[x.0].len()
                && (grid[x.0][x.1] <= (grid[cur_pos.0][cur_pos.1] + 1))
        })
        .collect()
}

fn generate_neighbours(cur_pos: (usize, usize)) -> Vec<(usize, usize)> {
    let mut neighbours = vec![];
    neighbours.push((cur_pos.0, cur_pos.1 + 1));
    if cur_pos.1 != 0 {
        neighbours.push((cur_pos.0, cur_pos.1 - 1));
    }

    neighbours.push((cur_pos.0 + 1, cur_pos.1));
    if cur_pos.0 != 0 {
        neighbours.push((cur_pos.0 - 1, cur_pos.1));
    }

    neighbours
}
