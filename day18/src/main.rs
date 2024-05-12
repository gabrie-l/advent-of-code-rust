use std::ops::Add;
macro_rules! read_input {
    ($name: expr) => {{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open($name).expect("File not found");
        let reader = BufReader::new(f);
        let f_data: Vec<Vec<String>> = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .split_whitespace()
                    .map(|s| s.to_owned())
                    .collect()
            })
            .collect();
        let directions: Vec<String> = f_data.iter().map(|row| row[0].clone()).collect();
        let quantifiers: Vec<String> = f_data.iter().map(|row| row[1].clone()).collect();
        let colors: Vec<String> = f_data
            .iter()
            .map(|row| row[2].trim_matches(|c| c == '(' || c == ')').to_string())
            .collect();
        (directions, quantifiers, colors)
    }};
}

#[derive(Copy, Clone, Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
    fn add_dir(&self, dir: &Direction) -> Self {
        *self + dir.to_point()
    }
    // add code here
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Left,
    Down,
    Up,
}
impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "D" => Self::Down,
            "U" => Self::Up,
            "L" => Self::Left,
            "R" => Self::Right,
            _ => unreachable!(),
        }
    }
}
impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Down,
            3 => Self::Up,
            2 => Self::Left,
            0 => Self::Right,
            _ => unreachable!(),
        }
    }
}

impl Direction {
    fn to_point(&self) -> Point {
        match self {
            Self::Up => Point { x: -1, y: 0 },
            Self::Down => Point { x: 1, y: 0 },
            Self::Right => Point { x: 0, y: 1 },
            Self::Left => Point { x: 0, y: -1 },
        }
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn dig(dir: Direction, quant: usize, acc: &mut Vec<Point>) {
    if acc.is_empty() {
        acc.push(Point::new(0, 0))
    }
    for _ in 0..quant {
        acc.push(acc.last().unwrap().add_dir(&dir))
    }
}

fn volume<A, P>(points: &Vec<Point>, area_fn: A, picks_fn: P) -> i64
where
    A: Fn(&Vec<Point>) -> i64,
    P: Fn(i64, i64) -> i64,
{
    let area = area_fn(points);
    picks_fn(area, points.len() as i64) + points.len() as i64
}

fn main() {
    let (_directions, _quantifiers, colors) = read_input!("input");
    let mut directions = Vec::new();
    let mut quantifiers = Vec::new();
    colors.iter().for_each(|c| {
        directions.push(Direction::from(c.chars().last().unwrap() as u8 - b'0'));
        quantifiers.push(usize::from_str_radix(&c[1..6], 16).unwrap());
    });

    let mut points: Vec<Point> = Vec::new();
    for i in 0..directions.len() {
        dig(directions[i].clone(), quantifiers[i], &mut points)
    }
    println!("total point count: {} ", points.len(),);

    let v = volume(
        &points,
        |v| {
            let mut acc = 0i64;
            for i in 0..v.len() {
                let Point { x: x0, y: y0 } = v[i];
                let Point { x: x1, y: y1 } = v[(i + 1) % v.len()];
                acc += (y0 + y1) * (x1 - x0);
            }
            acc / 2
        },
        |a, b| a - b / 2 + 1,
    );
    println!("total points {:?}", v);
}
