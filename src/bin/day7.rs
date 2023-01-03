use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

type DirHandle = Rc<RefCell<Dir>>;

#[derive(Debug, Default)]
struct Dir {
    name: String,
    size: usize,
    parent: Option<DirHandle>,
    dirs: HashMap<String, DirHandle>,
}

fn update_sizes(curr: DirHandle) -> usize {
    if !curr.borrow().dirs.is_empty() {
        let dir_sizes = curr
            .borrow()
            .dirs
            .values()
            .fold(0, |acc, val| acc + update_sizes(Rc::clone(val)));
        curr.borrow_mut().size += dir_sizes;
    }
    curr.borrow().size
}

fn get_dir_sizes(curr: DirHandle) -> Vec<usize> {
    if curr.borrow().dirs.is_empty() {
        vec![curr.borrow().size]
    } else {
        let mut res = curr
            .borrow()
            .dirs
            .values()
            .flat_map(|val| get_dir_sizes(Rc::clone(val)))
            .collect::<Vec<_>>();
        res.push(curr.borrow().size);
        res
    }
}

fn parse_dirs() -> Rc<RefCell<Dir>> {
    let lines = include_str!("../../input/day7").lines();
    let head = Rc::new(RefCell::new(Dir::default()));
    let mut cwd = Rc::clone(&head);
    for line in lines {
        if line.starts_with("$ cd /") {
            cwd.borrow_mut().name = "/".to_string();
        } else if line.starts_with("$ cd ..") {
            let new_cwd = cwd.borrow().parent.clone().unwrap();
            cwd = new_cwd;
        } else if line.starts_with("$ cd") {
            let dir_name = line.split_at(5).1;
            let new_cwd = Rc::clone(&cwd.borrow().dirs[dir_name]);
            cwd = new_cwd;
        } else if line.starts_with("dir") {
            let dir_name = line.split_at(4).1;
            cwd.borrow_mut().dirs.insert(
                dir_name.to_string(),
                Rc::new(RefCell::new(Dir {
                    name: dir_name.to_string(),
                    parent: Some(Rc::clone(&cwd)),
                    size: 0,
                    dirs: HashMap::new(),
                })),
            );
        } else if line.starts_with("$ ls") {
        } else {
            let (size_s, _) = line.split_once(' ').unwrap();
            let size = size_s.parse::<usize>().unwrap();
            cwd.borrow_mut().size += size;
        }
    }
    head
}

fn main() {
    let head = parse_dirs();
    let total_size = update_sizes(Rc::clone(&head));
    let dir_sizes = get_dir_sizes(Rc::clone(&head));
    let p1 = dir_sizes
        .iter()
        .filter(|x| **x <= 100000_usize)
        .sum::<usize>();
    println!("sum of dirs (p1): {p1}");

    let free_space = 70000000 - total_size;
    let minimum_to_free = 30000000 - free_space;

    let mut best_size_to_free = total_size;
    for size in dir_sizes {
        if size > minimum_to_free && size < best_size_to_free {
            best_size_to_free = size;
        }
    }
    println!("best to free (p2): {best_size_to_free}");
}
