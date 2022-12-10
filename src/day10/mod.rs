pub fn run(input: String) -> (String, String) {
    let mut x: i32 = 1;
    let mut cycle = 0;
    let mut sum = 0;
    let interesting_cycles = vec![20,60,100,140,180,220];
    let mut display = vec![];
    let mut row = String::new();
    for line in input.lines() {
        cycle += 1;
        if interesting_cycles.contains(&cycle) {
            sum += x*cycle;
        }
        if (((cycle % 40)-1) - x).abs() < 2 {
            row += "#";
        } else {
            row += ".";
        }
        if (cycle % 40) == 0 {
            display.push(row);
            row = String::new();
        }
        if let Some((cmd, num)) = line.split_once(" ") {
            cycle += 1;
            if interesting_cycles.contains(&cycle) {
                sum += x*cycle;
            }
            if (((cycle % 40)-1) - x).abs() < 2 {
                row += "#";
            } else {
                row += ".";
            }
            if (cycle % 40) == 0 {
                display.push(row);
                row = String::new();
            }
            x += num.parse::<i32>().unwrap();
        } else {
            continue
        }
    }

    (format!("{}", sum).to_owned(),display.join("\n").to_owned())
}