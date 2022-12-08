use std::{vec, cmp::Ordering};

pub fn run(input: String) -> (String, String) {
    let mut grid: Vec<Vec<u32>> = vec![];
    for line in input.lines() {
        let mut linevec = vec![];
        for height in line.chars() {
            let height = height.to_digit(10).unwrap();
            linevec.push(height);
        }
        grid.push(linevec);
    }
    let mut visible = grid.len() * 2 + (grid[0].len() - 2) * 2;
    let mut highest_viewing_distance = 0;
    for i in 1..grid.len() - 1 {
        for j in 1..grid[0].len() - 1 {
            if is_visible(i, j, &grid) {
                visible += 1;
            }
            let view_distance = view_distance(i, j, &grid);
            if view_distance > highest_viewing_distance {
                highest_viewing_distance = view_distance;
            }
        }
    }
    (visible.to_string(), highest_viewing_distance.to_string())
}

fn view_distance(i: usize, j: usize, grid: &[Vec<u32>]) -> u32 {
    check_row_distance(i, j, grid) * check_col_distance(i, j, grid)
}

fn check_col_distance(i: usize, j: usize, grid: &[Vec<u32>]) -> u32 {
    let height = grid[i][j];
    let mut up = 0;
    let mut down = 0;
    for (row, item) in grid.iter().enumerate() {
        match row.cmp(&i) {
            Ordering::Less => {
            if item[j] >= height {
                up = 0;
            }
            up += 1;
        },
        Ordering::Greater => {
            down += 1;
            if item[j] >= height {
                break;
            }
        },
        _ => (),
    }
}
    up * down
}

fn check_row_distance(i: usize, j: usize, grid: &[Vec<u32>]) -> u32 {
    let height = grid[i][j];
    let mut left = 0;
    let mut right = 0;
    for col in 0..grid[0].len() {
        match col.cmp(&j) {
            Ordering::Less => {
                if grid[i][col] >= height {
                    left = 0;
                }
                left += 1;
            },
            Ordering::Greater => {
                right += 1;
                if grid[i][col] >= height {
                    break;
                }
            },
            _ => (),
    }
    }
    left * right
}

fn is_visible(i: usize, j: usize, grid: &[Vec<u32>]) -> bool {
    if check_row_visibility(i, j, grid) {
        return true;
    };

    if check_col_visibility(i, j, grid) {
        return true;
    };
    false
}

fn check_col_visibility(i: usize, j: usize, grid: &[Vec<u32>]) -> bool {
    let mut highest = 0;
    let height = grid[i][j];
    for (row, item) in grid.iter().enumerate() {
        if row == i {
            if highest < height {
                return true;
            }
            highest = 0;
        } else if item[j] > highest {
            highest = item[j];
        }
    }
    if highest < height {
        return true;
    }
    false
}

fn check_row_visibility(i: usize, j: usize, grid: &[Vec<u32>]) -> bool {
    let mut highest = 0;
    let height = grid[i][j];
    for col in 0..grid[0].len() {
        if col == j {
            if highest < height {
                return true;
            }
            highest = 0;
        } else if grid[i][col] > highest {
            highest = grid[i][col];
        }
    }
    if highest < height {
        return true;
    }
    false
}
