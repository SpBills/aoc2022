use std::fs;

type Data = Vec<(String, usize)>;

const TOTAL_SPACE: usize = 70000000;
const NEEDED_SPACE: usize = 30000000;

fn main() {
    let data = fs::read_to_string("./inp/d7.txt").unwrap();

    let mut found: Data = Vec::new();
    let mut all_found: Data = Vec::new();

    let mut dir_stack: Data = Vec::new();
    for line in data.lines() {
        let split = line.split_whitespace();

        match split.collect::<Vec<_>>().as_slice() {
            ["$", "cd", ".."] => {
                let (name, size) = dir_stack.pop().unwrap();
                let (lname, lsize) = dir_stack.last().unwrap().clone();
                dir_stack.pop();
                dir_stack.push((lname, lsize + size));

                if size <= 100_000 {
                    found.push((name.clone(), size))
                }

                all_found.push((name, size))
            }
            ["$", "cd", x] => dir_stack.push((x.to_string(), 0)),
            ["$", "ls"] => continue,
            ["dir", x] => continue,
            [x, _] => {
                let x = x.parse::<usize>().unwrap();
                let (lname, lsize) = dir_stack.last().unwrap().clone();
                dir_stack.pop();
                dir_stack.push((lname, lsize + x))
            }
            _ => unreachable!(),
        }
    }

    let s: usize = found.iter().map(|x| x.1).sum();
    println!("{}", s);

    // collapse the dir stack
    let mut f = ("".to_string(), 0);
    while let Some(l) = dir_stack.pop() {
        if !dir_stack.is_empty() {
            let (lname, lsize) = dir_stack.last().unwrap();
            f = (lname.clone(), lsize + l.1);
            dir_stack.pop();
            dir_stack.push(f.clone());
        }
    }

    let remaining = TOTAL_SPACE - f.1;
    let min = all_found.iter().map(|f| (f.1, remaining + f.1 >= NEEDED_SPACE)).filter(|(_, b)| *b).min().unwrap();

    println!("{}", min.0);
}
