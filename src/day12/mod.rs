use std::collections::HashMap;

pub fn run(input: String) -> (String, String) {
    let mut grid: Vec<Vec<u32>> = vec![];
    let mut start_pos = (0, 0);
    let mut goal_pos = (0, 0);
    let mut p2_starts = vec![];
    for (y, line) in input.lines().enumerate() {
        let mut grid_row = vec![];
        for (x, letter) in line.chars().enumerate() {
            if letter == 'S' {
                start_pos = (y, x);
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
    let path = find_goal(&grid, start_pos, goal_pos, &mut visited);
    let mut shortest = 5000;

    for start in p2_starts {
        let path = find_goal(&grid, start, goal_pos, &mut visited);
        if path < shortest {
            shortest = path;
        }
    }


    (format!("{path}"), format!("{shortest}"))
}

fn find_goal(
    grid: &Vec<Vec<u32>>,
    cur_pos: (usize, usize),
    goal_pos: (usize, usize),
    visited: &mut HashMap<(usize, usize), u32>,
) -> u32 {
    if cur_pos == goal_pos {
        return 1;
    }

    if visited.contains_key(&cur_pos) {
        return visited[&cur_pos];
    }

    visited.insert(cur_pos, 5000);

    let mut possible_moves = find_possible_moves(grid, cur_pos);
    possible_moves.sort_by_key(|a| manhattan(a, &goal_pos));
    let mut min = 5000;
    for new_pos in possible_moves {
        let path = find_goal(grid, new_pos, goal_pos, visited);
        if path <= min {
            min = path;
        }
    }
    visited.insert(cur_pos, min.saturating_add(1));
    min.saturating_add(1)
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
