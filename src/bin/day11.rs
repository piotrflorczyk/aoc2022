use std::cell::RefCell;

struct Monkey {
    items: RefCell<Vec<u64>>,
    divider: u64,
    throw_true: usize,
    throw_false: usize,
    operation: Box<dyn Fn(u64) -> u64>,
}

fn parse_input() -> Vec<Monkey> {
    let input = include_str!("../../input/day11")
        .split("Monkey")
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>();
    input
        .iter()
        .map(|monkey_data| {
            let lines = monkey_data.split('\n').collect::<Vec<_>>();
            let items = lines[1]
                .split_once(": ")
                .unwrap()
                .1
                .split(", ")
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let divider = lines[3]
                .split_once("divisible by ")
                .unwrap()
                .1
                .parse::<u64>()
                .unwrap();
            let throw_true = lines[4]
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();
            let throw_false = lines[5]
                .split_once("monkey ")
                .unwrap()
                .1
                .parse::<usize>()
                .unwrap();

            if lines[2].contains("old * old") {
                Monkey {
                    items: RefCell::new(items),
                    divider,
                    throw_false,
                    throw_true,
                    operation: Box::new(|x| x * x),
                }
            } else if lines[2].contains('+') {
                let add = lines[2].split_once("+ ").unwrap().1.parse::<u64>().unwrap();
                Monkey {
                    items: RefCell::new(items),
                    divider,
                    throw_false,
                    throw_true,
                    operation: Box::new(move |x| x + add),
                }
            } else {
                let mul = lines[2].split_once("* ").unwrap().1.parse::<u64>().unwrap();
                Monkey {
                    items: RefCell::new(items),
                    divider,
                    throw_false,
                    throw_true,
                    operation: Box::new(move |x| x * mul),
                }
            }
        })
        .collect::<Vec<_>>()
}

fn run_monkeys(monkeys: &Vec<Monkey>, rounds: usize, div: u64) -> usize {
    let mut inspections = vec![0; monkeys.len()];
    for _ in 0..rounds {
        monkeys.iter().enumerate().for_each(|(idx, monkey)| {
            inspections[idx] += monkey.items.borrow().len();
            monkey.items.borrow().iter().for_each(|item| {
                let worry_level = if div == 0 {
                    (monkey.operation)(*item) / 3
                } else {
                    (monkey.operation)(*item) % div
                };

                let throw_to = if worry_level % monkey.divider == 0 {
                    monkey.throw_true
                } else {
                    monkey.throw_false
                };
                monkeys[throw_to].items.borrow_mut().push(worry_level);
            });
            monkey.items.borrow_mut().clear();
        });
    }
    inspections.sort();
    inspections.reverse();
    inspections[0] * inspections[1]
}

fn main() {
    let monkeys = parse_input();
    let p1 = run_monkeys(&monkeys, 20, 0);
    println!("p1: {p1}");

    let monkeys_p2 = parse_input();
    let div = monkeys_p2.iter().map(|m| m.divider).product();
    let p2 = run_monkeys(&monkeys_p2, 10000, div);
    println!("p2: {p2}");
}
