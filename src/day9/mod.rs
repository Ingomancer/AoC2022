use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
}

#[derive(Copy, Clone)]
struct Move {
    axis: Axis,
    distance: i32,
}

pub fn run(input: String) -> (String, String) {
    let mut rope = vec![Point { x: 0, y: 0 }; 10];
    let mut visited = HashSet::new();
    let mut visited2 = HashSet::new();
    for line in input.lines() {
        let (direction, distance) = line.split_once(' ').unwrap();
        let direction = match direction {
            "U" => Move {
                axis: Axis::Y,
                distance: 1,
            },
            "D" => Move {
                axis: Axis::Y,
                distance: -1,
            },
            "L" => Move {
                axis: Axis::X,
                distance: -1,
            },
            "R" => Move {
                axis: Axis::X,
                distance: 1,
            },
            _ => Move {
                axis: Axis::Y,
                distance: 0,
            },
        };
        for _ in 0..distance.parse().unwrap() {
            for i in 0..rope.len() - 1 {
                let actual_direction = if i > 0 {
                    Move {
                        axis: Axis::Y,
                        distance: 0,
                    }
                } else {
                    direction
                };

                let mut head = rope[i];
                let mut tail = rope[i + 1];
                (head, tail) = move_rope(head, tail, &actual_direction);
                rope[i] = head;
                rope[i + 1] = tail;
                if i == 0 {
                    visited.insert(tail);
                }
                if i == rope.len() - 2 {
                    visited2.insert(tail);
                }
            }
        }
    }
    (
        format!("{}", visited.len()),
        format!("{}", visited2.len()),
    )
}

fn move_rope(head: Point, tail: Point, direction: &Move) -> (Point, Point) {
    let head = match direction.axis {
        Axis::X => Point {
            x: head.x + direction.distance,
            ..head
        },
        Axis::Y => Point {
            y: head.y + direction.distance,
            ..head
        },
    };
    let tail = move_tail(head, tail);

    (head, tail)
}

fn move_tail(head: Point, tail: Point) -> Point {
    if tail == head {
        return tail;
    }
    let mut new_tail = tail;
    let distance = (head.x - tail.x).abs() + (head.y - tail.y).abs();
    if distance >= 3 {
        new_tail.x += (head.x - tail.x).signum();
        new_tail.y += (head.y - tail.y).signum();
    } else if distance == 2 {
        if tail.x == head.x {
            new_tail.y += (head.y - tail.y).signum();
        } else if tail.y == head.y {
            new_tail.x += (head.x - tail.x).signum();
        }
    }
    new_tail
}
