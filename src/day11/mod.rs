struct Monkey {
    items: Vec<u128>,
    items2: Vec<u128>,
    oper: Box<dyn Fn(u128) -> u128>,
    test: Box<dyn Fn(u128) -> usize>,
}

pub fn run(input: String) -> (String, String) {
    let mut monkeys: Vec<Monkey> = vec![];
    let mut lines = input.lines().peekable();
    let mut inspected = vec![];
    let mut inspected2 = vec![];
    let mut divider = 1;
    while lines.peek().is_some() {
        lines.next();
        let starting_items: Vec<u128> = get_items(lines.next().unwrap());
        let oper = get_oper(lines.next().unwrap());
        let test = get_test(lines.next().unwrap());
        divider *= test;
        let true_target = get_target(lines.next().unwrap());
        let false_target = get_target(lines.next().unwrap());
        lines.next();

        monkeys.push(Monkey {
            items: starting_items.clone(),
            items2: starting_items,
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
        inspected2.push(0);
    }
    for round in 0..10000 {
        for monkey in 0..monkeys.len() {
            if round < 20 {
                while !monkeys[monkey].items.is_empty() {
                    inspected[monkey] += 1;
                    let mut item = monkeys[monkey].items.remove(0);
                    item = (monkeys[monkey].oper)(item);
                    item /= 3;
                    let target = (monkeys[monkey].test)(item);
                    monkeys[target].items.push(item);
                }
            }
            while !monkeys[monkey].items2.is_empty() {
                inspected2[monkey] += 1;
                let mut item = monkeys[monkey].items2.remove(0);
                item %= divider;
                item = (monkeys[monkey].oper)(item);
                let target = (monkeys[monkey].test)(item);
                monkeys[target].items2.push(item);
            }
        }
    }
    inspected.sort();
    let highest: u128 = inspected.pop().unwrap();
    let second_highest = inspected.pop().unwrap();
    inspected2.sort();
    let highest2: u128 = inspected2.pop().unwrap();
    let second_highest2 = inspected2.pop().unwrap();

    (
        format!("{}", highest * second_highest),
        format!("{}", highest2 * second_highest2),
    )
}

fn get_items(str: &str) -> Vec<u128> {
    str.split_once(": ")
        .unwrap()
        .1
        .split(", ")
        .map(|x| x.parse::<u128>().unwrap())
        .collect()
}

fn get_oper(str: &str) -> Box<dyn Fn(u128) -> u128> {
    let oper = str.split_once("old ").unwrap().1.split_once(' ').unwrap();
    let num = oper.1.parse::<u128>();
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

fn get_test(str: &str) -> u128 {
    str.split_once("by ").unwrap().1.parse().unwrap()
}

fn get_target(str: &str) -> usize {
    str.split_once("y ").unwrap().1.parse().unwrap()
}
