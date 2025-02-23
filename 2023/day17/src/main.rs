use std::collections::BinaryHeap;
use std::collections::HashMap;

macro_rules! read_input {
    ($name:expr) => {{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open($name).expect("File not found");
        let reader = BufReader::new(f);
        let lines: Vec<Vec<u8>> = reader
            .lines()
            .map(|l| l.unwrap().chars().map(|c| c as u8 - b'0').collect())
            .collect();
        lines
    }};
}
#[derive(PartialOrd, Ord, Debug, PartialEq, Hash, Eq, Clone)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}

impl Direction {
    fn opp(&self) -> Direction {
        match self {
            Direction::Right => Direction::Left,
            Direction::Left => Direction::Right,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(PartialOrd, Ord, Debug, Hash, Clone, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn adj_pts(&self, MAX_ROW: usize, MAX_COL: usize) -> Vec<(Point, Direction)> {
        let mut res = Vec::new();
        if self.x > 0 {
            res.push((Point::new(self.x - 1, self.y), Direction::Up))
        }
        if self.y > 0 {
            res.push((Point::new(self.x, self.y - 1), Direction::Left))
        }
        if self.x < MAX_ROW - 1 {
            res.push((Point::new(self.x + 1, self.y), Direction::Down))
        }
        if self.y < MAX_COL - 1 {
            res.push((Point::new(self.x, self.y + 1), Direction::Right))
        }
        res
    }
}

#[derive(PartialOrd, Ord, Debug, Hash, Clone, PartialEq, Eq)]
struct Node {
    dir: Direction,
    position: Point,
    count: usize,
}

impl Node {
    fn new(dir: Direction, position: Point, count: usize) -> Self {
        Self {
            dir,
            position,
            count,
        }
    }
}

fn adj_nodes<const MIN: usize, const MAX: usize>(node: &Node, grid: &[Vec<u8>]) -> Vec<Node> {
    let mut adj_nodes = Vec::new();
    let MAX_ROW: usize = grid.len();
    let MAX_COL: usize = grid[0].len();
    for (pos, dir) in node.position.adj_pts(MAX_ROW, MAX_COL) {
        if dir == node.dir.opp() {
            continue;
        } else if dir != node.dir && node.count >= MIN {
            adj_nodes.push(Node::new(dir, pos, 1))
        } else if dir == node.dir && node.count < MAX {
            adj_nodes.push(Node::new(dir, pos, node.count + 1))
        }
    }
    adj_nodes
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct State {
    cost: usize,
    node: Node,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra<F, G>(grid: &[Vec<u8>], start: &Point, goal_fn: G, neighbor_fn: F) -> usize
where
    F: Fn(&Node, &[Vec<u8>]) -> Vec<Node>,
    G: Fn(&Node) -> bool,
{
    let mut seen = HashMap::new();
    seen.insert(Node::new(Direction::Down, start.clone(), 0), 0);
    seen.insert(Node::new(Direction::Right, start.clone(), 0), 0);
    let mut pq = BinaryHeap::new();
    pq.push(State {
        cost: 0,
        node: Node::new(Direction::Down, start.clone(), 0),
    });
    pq.push(State {
        cost: 0,
        node: Node::new(Direction::Right, start.clone(), 0),
    });

    while let Some(State { cost, node }) = pq.pop() {
        if goal_fn(&node) {
            return cost;
        }

        for adj in neighbor_fn(&node, grid) {
            let new_cost = cost + grid[adj.position.x][adj.position.y] as usize;
            if let Some(&best) = seen.get(&adj) {
                if new_cost >= best {
                    continue;
                }
            }
            seen.insert(adj.clone(), new_cost);
            pq.push(State {
                cost: new_cost,
                node: adj,
            });
        }
    }
    todo!()
}

fn main() {
    let grid = read_input!("input");
    let start = Point::new(0, 0);
    let goal = Point::new(grid.len() - 1, grid[0].len() - 1);
    let p1 = dijkstra(&grid, &start, |n| n.position == goal, adj_nodes::<1, 3>);
    let p2 = dijkstra(
        &grid,
        &start,
        |n| n.position == goal && n.count >= 4,
        adj_nodes::<4, 10>,
    );

    println!("res: {p1}");
    println!("res: {p2}");
}
