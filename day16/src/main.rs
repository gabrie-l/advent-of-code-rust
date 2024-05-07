macro_rules! read_input {
    ($name:expr) => {{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open($name).expect("File not found");
        let reader = BufReader::new(f);
        let lines: Vec<Vec<char>> = reader
            .lines()
            .map(|l| l.unwrap().chars().collect())
            .collect();
        lines
    }};
}
#[derive(PartialEq, Eq, Clone)]
enum Direction {
    Left,
    Right,
    Down,
    Up,
}
impl Direction {
    fn to_idx(&self) -> usize {
        match self {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        }
    }
}
#[derive(Clone)]
struct Coordinate {
    x: usize,
    y: usize,
}
struct Cave<'a> {
    board: &'a [Vec<char>],
    seen: &'a mut [Vec<bool>],
}

fn move_beam(
    coords: Coordinate,
    cave: &mut Cave,
    dir: Direction,
    dp: &mut Vec<Vec<Vec<u16>>>,
) -> u16 {
    let Coordinate { x: row, y: col } = coords;
    let didx = dir.to_idx();
    if dp[row][col][didx] < u16::MAX {
        return 0;
    }
    let curr_pos = cave.board[row][col];
    // let mut new_dirs: Vec<Direction> = Vec::new();
    let new_dirs: Vec<Direction> = match (&dir, curr_pos) {
        (_, '.') => [dir].to_vec(), //new_dirs.push(dir),
        (d, '-') => match d {
            direction if *direction == Direction::Left || *direction == Direction::Right => {
                [dir].to_vec()
                // new_dirs.push(dir)
            }
            _ => {
                [Direction::Left, Direction::Right].to_vec()
                // new_dirs.push(Direction::Left);
                // new_dirs.push(Direction::Right);
            }
        },
        (d, '|') => match d {
            direction if *direction == Direction::Up || *direction == Direction::Down => {
                [dir].to_vec()
                // new_dirs.push(dir)
            }
            _ => {
                [Direction::Up, Direction::Down].to_vec()
                // new_dirs.push(Direction::Up);
                // new_dirs.push(Direction::Down);
            }
        },
        (d, '/') => match *d {
            Direction::Right => [Direction::Up].to_vec(), //new_dirs.push(Direction::Up),
            Direction::Up => [Direction::Right].to_vec(), //new_dirs.push(Direction::Right),
            Direction::Left => [Direction::Down].to_vec(), //new_dirs.push(Direction::Down),
            _ => [Direction::Left].to_vec(),              //new_dirs.push(Direction::Left),
        },
        (d, '\\') => match *d {
            Direction::Right => [Direction::Down].to_vec(), //new_dirs.push(Direction::Down),
            Direction::Down => [Direction::Right].to_vec(), //new_dirs.push(Direction::Right),
            Direction::Left => [Direction::Up].to_vec(),    //new_dirs.push(Direction::Up),
            _ => [Direction::Left].to_vec(),                // new_dirs.push(Direction::Left),
        },

        _ => {
            vec![]
        }
    };
    let mut acc = 0u16;
    if !cave.seen[row][col] {
        acc += 1;
        cave.seen[row][col] = true
    }
    dp[row][col][didx] = acc;
    for dir in new_dirs {
        match dir {
            Direction::Left => {
                let (x, y) = (coords.x, coords.y.checked_sub(1).unwrap_or(200));
                if y != 200 {
                    acc += move_beam(Coordinate { x, y }, cave, dir, dp)
                }
            }
            Direction::Right => {
                let (x, y) = (coords.x, coords.y + 1);
                if y < cave.board[0].len() {
                    acc += move_beam(Coordinate { x, y }, cave, dir, dp)
                }
            }
            Direction::Up => {
                let (x, y) = (coords.x.checked_sub(1).unwrap_or(200), coords.y);
                if x != 200 {
                    acc += move_beam(Coordinate { x, y }, cave, dir, dp)
                }
            }
            Direction::Down => {
                let (x, y) = (coords.x + 1, coords.y);
                if x < cave.board.len() {
                    acc += move_beam(Coordinate { x, y }, cave, dir, dp)
                }
            }
        }
    }
    dp[row][col][didx] = acc;
    acc
}

fn main() {
    let map = read_input!("input");

    let mut coord_list: Vec<Coordinate> = Vec::new();
    (0..map.len()).for_each(|i| {
        coord_list.push(Coordinate {
            x: i,
            y: map[0].len() - 1,
        });
        coord_list.push(Coordinate { x: i, y: 0 })
    });
    (1..map[0].len() - 1).for_each(|i| {
        coord_list.push(Coordinate { x: 0, y: i });
        coord_list.push(Coordinate {
            x: map.len() - 1,
            y: i,
        })
    });
    let mut acc: u16 = 0;
    for c in coord_list {
        let Coordinate { x: x1, y: y1 } = c;
        let mut directions: Vec<Direction> = Vec::new();
        match x1 {
            0 => directions.push(Direction::Down),
            x if x == map.len() - 1 => directions.push(Direction::Up),
            _ => {}
        }
        match y1 {
            0 => directions.push(Direction::Right),
            y if y == map[0].len() - 1 => directions.push(Direction::Left),
            _ => {}
        }
        for d in &directions {
            let mut dp: Vec<Vec<Vec<u16>>> = vec![vec![vec![u16::MAX; 4]; map[0].len()]; map.len()];
            let mut cave = Cave {
                board: &map,
                seen: &mut vec![vec![false; map[0].len()]; map.len()],
                // energy: &mut energy,
            };
            let tmp = move_beam(c.to_owned(), &mut cave, d.to_owned(), &mut dp);
            acc = acc.max(tmp);
        }
    }

    println!("total: {acc}");
}
