use std::fs;

type Stack<T> = Vec<T>;

#[derive(Debug)]
struct Board(Vec<Stack<char>>);

impl Board {
    fn new() -> Self {
        Self((1..10).map(|_| Stack::new()).collect::<Vec<Stack<char>>>())
    }
}

const idc: [usize; 9] = [1, 5, 9, 13, 17, 21, 25, 29, 33];

/// Move #, from, to
#[derive(Debug, Clone, Copy)]
struct Move(usize, usize, usize);

fn main() {
    let data = fs::read_to_string("./inp/d5.txt").unwrap();

    let mut lines = data.lines();
    // parse the board
    let mut board = Board::new();
    while let Some(line) = lines.next() {
        if line.chars().nth(1).unwrap() == '1' {
            break;
        }

        for (i, idx) in idc.iter().enumerate() {
            let c = line.chars().nth(*idx).unwrap();
            if !c.is_whitespace() {
                board.0[i].push(c);
            }
        }

        println!("{}", line);
    }

    board.0.iter_mut().for_each(|s| s.reverse());

    // empty line
    lines.next();

    // parse in the moves
    let mut moves: Vec<Move> = Vec::new();
    while let Some(line) = lines.next() {
        let ws = line.split_whitespace().collect::<Vec<&str>>();
        let num = ws[1].parse::<usize>().unwrap();
        let from = ws[3].parse::<usize>().unwrap()-1;
        let to = ws[5].parse::<usize>().unwrap()-1;

        moves.push(Move(num, from, to));
    }

    // do the moves
    for m in moves {
        let ref mut from_stack = board.0[m.1];

        let mut v: Vec<char> = vec![];
        for _ in 0..m.0 {
            let t = from_stack.pop().unwrap();
            v.push(t);
        }

        board.0[m.2].append(&mut v);
    }

    // get top of each stack
    for s in board.0 {
        print!("{}", s.last().unwrap());
    }
}
