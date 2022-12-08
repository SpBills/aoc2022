use std::fs;

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone)]
struct Node {
    pos: usize,
    height: usize,
}

impl Node {
    fn new(pos: usize, height: usize) -> Self {
        Self { pos, height }
    }
}

#[derive(Debug, Clone)]
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
        if n.pos % self.row == 98 {
            return None;
        }
        self.nodes.get(n.pos + 1)
    }
}

/// `G` is our search graph
///
/// `n` is our node to find the next of.
///
/// `orig` tracks the original node to do comparisons
///
/// `dir` gives us the direction to work off of
fn node_is_visible_dir(G: &Graph, n: &Node, orig: &Node, dir: Dir) -> bool {
    let next_maybe = match dir {
        Dir::Up => G.up(n),
        Dir::Right => G.right(n),
        Dir::Down => G.down(n),
        Dir::Left => G.left(n),
    };

    let Some(next) = next_maybe else {
        return true;
    };

    if next.height >= orig.height {
        return false;
    }

    node_is_visible_dir(G, next, orig, dir)
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

    // G.nodes.iter().for_each(|n| print!("({},{})", n.c, n.pos));

    let num = G
        .nodes
        .iter()
        .filter(|n| {
            [Dir::Up, Dir::Right, Dir::Down, Dir::Left]
                .into_iter()
                .any(|dir| node_is_visible_dir(&G, n, n, dir))
        })
        .collect::<Vec<_>>()
        .len();

    println!("{}", num)
}
