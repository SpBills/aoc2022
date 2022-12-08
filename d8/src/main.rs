use std::fs;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left
}

#[derive(Debug)]
struct Node {
    pos: usize,
    height: usize,
}

impl Node {
    fn new(pos: usize, height: usize) -> Self {
        Self { pos, height }
    }
}

#[derive(Debug)]
struct Graph {
    /// row length
    row: usize,
    /// laid out in a single dimension array. mod 99 to find rows/col
    nodes: Vec<Node>,
}

impl Graph {
    fn new(row: usize, nodes: Vec<Node>) -> Self {
        Self { nodes, row }
    }

    fn up(&self, n: &Node) -> Option<&Node> {
        let pos = n.pos.checked_sub(self.row)?;
        self.nodes.get(pos)
    }

    fn down(&self, n: &Node) -> Option<&Node> {
        self.nodes.get(n.pos + self.row)
    }

    fn left(&self, n: &Node) -> Option<&Node> {
        if n.pos % self.row == 0 {
            return None;
        }
        let pos = n.pos.checked_sub(1)?;
        

        self.nodes.get(pos)
    }

    fn right(&self, n: &Node) -> Option<&Node> {
        if n.pos % (self.row - 1) == 0 {
            return None;
        }
        self.nodes.get(n.pos + 1)
    }
}

fn node_is_visible_dir(G: &Graph, n: &Node, dir: Dir) -> bool {
    let next_maybe: Option<&Node> = match dir {
        Dir::Up => G.up(n),
        Dir::Right => G.right(n),
        Dir::Down => G.down(n),
        Dir::Left => G.left(n),
    };

    let Some(next) = next_maybe else {
        return true;
    };

    if next.height > n.height {
        return false;
    }

    node_is_visible_dir(G, next, dir)
}

fn main() {
    let data = fs::read_to_string("./inp/d8.txt").unwrap();

    let row_size = data.lines().peekable().peek().unwrap().len();
    let correct_data = data.replace(&['\n', '\r'][..], "");
    let nodes = correct_data
        .chars()
        .enumerate()
        .map(|(i, c)| Node::new(i, c.to_digit(10).unwrap() as usize))
        .collect::<Vec<_>>();
    let G: Graph = Graph::new(row_size, nodes);

    let num = G.nodes.iter().filter(|n| [Dir::Up, Dir::Right, Dir::Down, Dir::Left].iter().any(|dir| node_is_visible_dir(&G, n, *dir))).collect::<Vec<_>>().len();

    println!("{}", num)
}
