use std::collections::HashSet;

#[derive(Eq, Hash, PartialEq, Clone)]
struct Coordinate {
    x: u32,
    y: u32,
}

pub fn run(input: String) -> (String, String) {
    let (grid, left, right, bottom) = make_grid(input);
    let (part1, part2) = drop_sand(grid, left, right, bottom);
    (format!("{part1}").to_owned(), format!("{part2}").to_owned())
}

fn drop_sand(mut grid: HashSet<Coordinate>, left: u32, right: u32, bottom: u32) -> (u32, u32) {
    let mut units = 0;
    let mut part1 = 0;
    let mut done = false;
    loop {
        let mut sand = Coordinate { x: 500, y: 0 };
        while in_grid(&sand, left, right, bottom, part1) {
            let mut next_sand = Coordinate {
                x: sand.x,
                y: sand.y + 1,
            };
            if grid.contains(&next_sand) {
                next_sand = Coordinate {
                    x: sand.x - 1,
                    y: next_sand.y,
                };
            }
            if grid.contains(&next_sand) {
                next_sand = Coordinate {
                    x: sand.x + 1,
                    y: next_sand.y,
                };
            }
            if grid.contains(&next_sand) {
                if part1 > 0 {
                    if sand == (Coordinate { x: 500, y: 0 }) {
                        done = true;
                        break;
                    }
                }
                grid.insert(sand.clone());
                break;
            }
            sand = next_sand;
        }
        if !in_grid(&sand, left, right, bottom, part1) && part1 == 0 {
            part1 = units;
            for i in 0..right * 2 {
                grid.insert(Coordinate {
                    x: i,
                    y: bottom + 2,
                });
            }
        }
        if done {
            break;
        }
        units += 1;
    }
    (part1, units)
}

fn in_grid(sand: &Coordinate, left: u32, right: u32, bottom: u32, part1: u32) -> bool {
    if part1 != 0 {
        return true;
    }
    if sand.x > left && sand.x < right && sand.y < bottom {
        true
    } else {
        false
    }
}

fn make_grid(input: String) -> (HashSet<Coordinate>, u32, u32, u32) {
    let mut grid = HashSet::new();
    let mut left = u32::MAX;
    let mut right = 0;
    let mut bottom = 0;
    for line in input.lines() {
        let mut prev_coord: Option<Coordinate> = None;
        for coordinate in line.split(" -> ") {
            let (x, y) = coordinate.split_once(",").unwrap();
            let x: u32 = x.parse().unwrap();
            let y: u32 = y.parse().unwrap();
            if x > right {
                right = x;
            }
            if x < left {
                left = x;
            }
            if y > bottom {
                bottom = y;
            }
            if let Some(prev_coord) = prev_coord {
                if x == prev_coord.x {
                    for i in y.min(prev_coord.y)..y.max(prev_coord.y) {
                        grid.insert(Coordinate { x, y: i });
                    }
                } else {
                    for i in x.min(prev_coord.x)..x.max(prev_coord.x) {
                        grid.insert(Coordinate { x: i, y });
                    }
                }
            }
            let coordinate = Coordinate { x, y };
            prev_coord = Some(coordinate.clone());
            grid.insert(coordinate);
        }
    }
    (grid, left, right, bottom)
}
