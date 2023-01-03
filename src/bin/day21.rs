use std::collections::HashMap;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Op {
    name: String,
    op1: String,
    op2: String,
    operatiron: u8,
}

fn parse() -> (HashMap<String, i64>, Vec<Op>) {
    let mut numbers = HashMap::new();
    let mut operations = Vec::new();

    include_str!("../../input/day21").lines().for_each(|line| {
        let split = line.split_once(": ").unwrap();
        let name = split.0;

        if line.contains('+') || line.contains('-') || line.contains('/') || line.contains('*') {
            let split2 = split.1.split(' ').collect::<Vec<_>>();
            operations.push(Op {
                name: name.to_string(),
                op1: split2[0].to_string(),
                op2: split2[2].to_string(),
                operatiron: split2[1].as_bytes()[0],
            });
        } else {
            let val = split.1.parse::<i64>().unwrap();
            numbers.insert(name.to_string(), val);
        }
    });
    (numbers, operations)
}

fn simplify(calc_cache: &mut HashMap<String, i64>, operations: &[Op]) -> Vec<Op> {
    let mut todo = operations.to_owned();
    loop {
        let mut new_todo = Vec::new();
        todo.iter().for_each(|v| {
            if calc_cache.contains_key(&v.op1) && calc_cache.contains_key(&v.op2) {
                let res = match v.operatiron {
                    b'+' => calc_cache[&v.op1] + calc_cache[&v.op2],
                    b'-' => calc_cache[&v.op1] - calc_cache[&v.op2],
                    b'/' => calc_cache[&v.op1] / calc_cache[&v.op2],
                    b'*' => calc_cache[&v.op1] * calc_cache[&v.op2],
                    _ => unreachable!(),
                };
                calc_cache.insert(v.name.clone(), res);
            } else {
                new_todo.push(v.clone());
            }
        });
        if todo.len() == new_todo.len() {
            break;
        } else {
            todo = new_todo;
        }
    }
    todo
}

fn reverse_simplify(calc_cache: &mut HashMap<String, i64>, operations: &[Op]) -> Vec<Op> {
    let mut todo = operations.to_owned();
    loop {
        let mut new_todo = Vec::new();
        todo.iter().for_each(|v| {
            if calc_cache.contains_key(&v.name) && calc_cache.contains_key(&v.op1) {
                let res = match v.operatiron {
                    b'+' => calc_cache[&v.name] - calc_cache[&v.op1],
                    b'-' => calc_cache[&v.op1] - calc_cache[&v.name],
                    b'/' => calc_cache[&v.op1] / calc_cache[&v.name],
                    b'*' => calc_cache[&v.name] / calc_cache[&v.op1],
                    _ => unreachable!(),
                };
                calc_cache.insert(v.op2.clone(), res);
            } else if calc_cache.contains_key(&v.name) && calc_cache.contains_key(&v.op2) {
                let res = match v.operatiron {
                    b'+' => calc_cache[&v.name] - calc_cache[&v.op2],
                    b'-' => calc_cache[&v.name] + calc_cache[&v.op2],
                    b'/' => calc_cache[&v.name] * calc_cache[&v.op2],
                    b'*' => calc_cache[&v.name] / calc_cache[&v.op2],
                    _ => unreachable!(),
                };
                calc_cache.insert(v.op1.clone(), res);
            } else {
                new_todo.push(v.clone());
            }
        });
        if todo.len() == new_todo.len() {
            break;
        } else {
            todo = new_todo;
        }
    }
    todo
}

fn day21_1() {
    let (mut calc_results, operations) = parse();
    simplify(&mut calc_results, &operations);
    let root = calc_results["root"];
    println!("root (p1): {root:?}");
}

fn day21_2() {
    let (mut calc_results, operations) = parse();
    calc_results.remove("humn");
    let remaining_operations = simplify(&mut calc_results, &operations);
    let root_op = remaining_operations
        .iter()
        .find(|x| x.name == "root")
        .unwrap();
    if calc_results.contains_key(&root_op.op1) {
        calc_results.insert(root_op.op2.clone(), calc_results[&root_op.op1]);
    } else {
        calc_results.insert(root_op.op1.clone(), calc_results[&root_op.op2]);
    }
    reverse_simplify(&mut calc_results, &remaining_operations);
    let humn = calc_results["humn"];
    println!("humn (p2): {humn:?}");
}

fn main() {
    day21_1();
    day21_2();
}
