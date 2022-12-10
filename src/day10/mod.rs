pub fn run(input: String) -> (String, String) {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let interesting_cycles = vec![20, 60, 100, 140, 180, 220];
    let mut display = vec!["\n".to_owned()];
    let mut row = String::new();
    let mut cell = -1;
    for line in input.lines() {
        let ticks;
        let mut num = 0;
        if let Some((cmd, add)) = line.split_once(" ") {
            num = add.parse().unwrap();
            ticks = 2;
        } else {
            ticks = 1;
        }
        for i in 0..ticks {
            cycle += 1;
            cell += 1;
            if interesting_cycles.contains(&cycle) {
                sum += x * cycle;
            }
            if (cell - x).abs() < 2 {
                row += "#";
            } else {
                row += ".";
            }
            if (cycle % 40) == 0 {
                cell = -1;
                display.push(row);
                row = String::new();
            }
        }
        x += num;
    }

    (format!("{}", sum).to_owned(), display.join("\n").to_owned())
}
