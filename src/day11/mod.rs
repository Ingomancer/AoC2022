struct Monkey {
    items: Vec<u64>,
    oper: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> usize>,
}

pub fn run(input: String) -> (String, String) {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut lines = input.lines().peekable();
    let mut inspected = vec![];
    while lines.peek().is_some() {
        lines.next();
        let starting_items: Vec<u64> = get_items(lines.next().unwrap());
        let oper = get_oper(lines.next().unwrap());
        let test = get_test(lines.next().unwrap());
        let true_target = get_target(lines.next().unwrap());
        let false_target = get_target(lines.next().unwrap());
        lines.next();
        monkeys.push(Monkey {
            items: starting_items,
            oper,
            test: Box::new(move |x| {
                if x % test == 0 {
                    true_target
                } else {
                    false_target
                }
            }),
        });
        inspected.push(0);
    }
    for _ in 0..20 {
        for monkey in 0..monkeys.len() {
            while monkeys[monkey].items.len() != 0 {
                inspected[monkey] += 1;
                let mut item = monkeys[monkey].items.remove(0);
                item = (monkeys[monkey].oper)(item);
                item = item / 3;
                let target = (monkeys[monkey].test)(item);
                monkeys[target].items.push(item);
            }
        }
    }
    let mut highest: u64 = 0;
    let mut second_highest: u64 = 0;
    for inspections in inspected {
        if inspections >= highest {
            second_highest = highest;
            highest = inspections;
        }
    }

    (
        format!("{}", highest * second_highest).to_owned(),
        "".to_owned(),
    )
}

fn get_items(str: &str) -> Vec<u64> {
    str.split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

fn get_oper(str: &str) -> Box<dyn Fn(u64) -> u64> {
    let oper = str.split_once("old ").unwrap().1.split_once(" ").unwrap();
    let num = oper.1.parse::<u64>();
    if oper.0 == "+" {
        match num {
            Ok(num) => Box::new(move |x| x + num),
            Err(_) => Box::new(move |x| x + x),
        }
    } else {
        match num {
            Ok(num) => Box::new(move |x| x * num),
            Err(_) => Box::new(move |x| x * x),
        }
    }
}

fn get_test(str: &str) -> u64 {
    str.split_once("by ").unwrap().1.parse().unwrap()
}

fn get_target(str: &str) -> usize {
    str.split_once("y ").unwrap().1.parse().unwrap()
}
