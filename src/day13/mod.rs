use std::cmp::Ordering;
use std::iter::zip;

#[derive(Debug, Clone)]
enum Elem {
    Int(u32),
    List(Vec<Elem>),
}

pub fn run(input: String) -> (String, String) {
    let mut inputiter = input.lines().peekable();
    let mut correct = 0;
    let mut index = 0;

    let div1 = parse_line("[[2]]");
    let div2 = parse_line("[[6]]");
    let mut div1_index = 1;
    let mut div2_index = 2;
    // 0.0742615
    // 5185, 23751
    while inputiter.peek().is_some() {
        index += 1;
        let line1 = inputiter.next().unwrap();
        let line2 = inputiter.next().unwrap();
        inputiter.next();
        let line1 = parse_line(line1);
        let line2 = parse_line(line2);

        if check_order(line1.clone(), div1.clone()) == Ordering::Less {
          div1_index += 1;
        }
        if check_order(line2.clone(), div1.clone()) == Ordering::Less {
          div1_index += 1;
        }
        if check_order(line1.clone(), div2.clone()) == Ordering::Less {
          div2_index += 1;
        }
        if check_order(line2.clone(), div2.clone()) == Ordering::Less {
          div2_index += 1;
        }
        match check_order(line1.clone(), line2.clone()) {
            Ordering::Less => {
                correct += index;
            }
            Ordering::Greater => {}
            Ordering::Equal => {
                println!("I don't think this should happen");
            }
        }
    }

    (
        format!("{correct}").to_owned(),
        format!("{}", div1_index*div2_index).to_owned(),
    )
}

fn check_order(line1: Elem, line2: Elem) -> Ordering {
    match line1.clone() {
        Elem::Int(x) => match line2 {
            Elem::Int(y) => x.cmp(&y),
            Elem::List(_) => check_order(Elem::List(vec![Elem::Int(x)]), line2),
        },
        Elem::List(x) => match line2 {
            Elem::List(y) => {
                let lencmp = x.len().cmp(&y.len());
                for (elem1, elem2) in zip(x, y) {
                    match check_order(elem1, elem2) {
                        Ordering::Less => return Ordering::Less,
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Equal => continue,
                    }
                }
                return lencmp;
            }
            Elem::Int(y) => check_order(line1, Elem::List(vec![Elem::Int(y)])),
        },
    }
}
fn parse_line(line: &str) -> Elem {
    let mut nesting = vec![];
    let mut nums = String::new();
    for char in line.chars() {
        if char == '[' {
            nesting.push(Elem::List(vec![]));
        } else if char == ']' {
            let elem;
            if nums.is_empty() {
                elem = nesting.pop().unwrap();
            } else {
                elem = Elem::Int(nums.parse().unwrap());
                nums = String::new();
            }
            if let Some(x) = nesting.last_mut() {
                match x {
                    Elem::List(l) => l.push(elem),
                    Elem::Int(_) => panic!("Should not happen"),
                }
            } else {
                return elem;
            }
        } else if char == ',' {
            let elem;
            if nums.is_empty() {
                elem = nesting.pop().unwrap();
            } else {
                elem = Elem::Int(nums.parse().unwrap());
            }
            if let Some(x) = nesting.last_mut() {
                match x {
                    Elem::List(l) => l.push(elem),
                    Elem::Int(_) => panic!("Should not happen"),
                }
            } else {
                nesting.push(elem);
            }
            nums = String::new();
        } else {
            nums.push(char);
        }
    }
    nesting.pop().unwrap()
}
