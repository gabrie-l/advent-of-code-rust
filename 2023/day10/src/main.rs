use maplit::hashmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn unroll(
    pipes: &HashMap<char, Vec<Vec<i32>>>,
    coords: &[usize],
    steps: u32,
    board: &mut Vec<Vec<char>>,
    p: &mut Vec<Vec<usize>>,
) -> Option<u32> {
    let loc = board[coords[0]][coords[1]];
    if loc == 'X' || loc == '.' {
        return None;
    }
    if loc == 'S' {
        if steps < 3 {
            return None;
        }
        println!("S Steps: {}", steps);
        let result = steps - 1;
        return Some(result - (result / 2));
    }
    p.push(vec![coords[0], coords[1]]);
    board[coords[0]][coords[1]] = 'X';
    if pipes.contains_key(&loc) {
        if let Some(deltas) = pipes.get(&loc) {
            for d in deltas {
                let n_row: Result<usize, _> = (coords[0] as i32 + d[0]).try_into();
                let n_col: Result<usize, _> = (coords[1] as i32 + d[1]).try_into();
                if let (Ok(x), Ok(y)) = (n_row, n_col) {
                    if let Some(s) = unroll(pipes, &[x, y], steps + 1, board, p) {
                        return Some(s);
                    };
                }
            }
        };
    };
    None
}

fn show_board(board: &Vec<Vec<char>>) {
    for b in board {
        println!("{:?}", b);
    }
}

fn interior_area(points: Vec<Vec<i32>>) -> u32 {
    let n = points.len();
    let mut area: i32 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        area += (points[i][0] * points[j][1]) - (points[j][0] * points[i][1]);
    }
    area.unsigned_abs() / 2
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();
    let pipes = hashmap! {
        '|' => vec![vec![-1,0],vec![1,0]],
        '-' => vec![vec![0,-1],vec![0,1]],
        'L' => vec![vec![-1,0],vec![0,1]],
        'J' => vec![vec![0,-1],vec![-1,0]],
        '7' => vec![vec![0,-1],vec![1,0]],
        'F' => vec![vec![1,0],vec![0,1]],
    };

    let mut board = lines.clone();
    let mut p: Vec<Vec<usize>> = Vec::new();
    for row in 0..lines.len() {
        for (col, c) in lines[row].iter().enumerate() {
            if *c == 'S' {
                p.push(vec![row, col]);
                let directions: Vec<Vec<i32>> =
                    vec![vec![-1, 0], vec![1, 0], vec![0, 1], vec![0, -1]];
                for d in directions {
                    let p_row: Result<usize, std::num::TryFromIntError> =
                        (row as i32 + d[0]).try_into();
                    let p_col: Result<usize, std::num::TryFromIntError> =
                        (col as i32 + d[1]).try_into();
                    if let (Ok(x), Ok(y)) = (p_row, p_col) {
                        let key: char = lines[x][y];
                        if pipes.contains_key(&key) {
                            if let Some(test) = unroll(&pipes, &[x, y], 1, &mut board, &mut p) {
                                println!("Max steps: {}", test);
                                // println!("{:?}", p);
                            };
                        }
                    }
                }
            }
        }
    }
    let p: Vec<Vec<i32>> = p
        .iter()
        .map(|p| p.iter().map(|&x| x as i32).collect::<Vec<i32>>())
        .collect();
    let point_count: u32 = p.len().try_into().unwrap();
    let area = interior_area(p);
    let inward_points = area + 1 - (point_count / 2);

    show_board(&board);
    println!("Total isles: {inward_points}");
}
