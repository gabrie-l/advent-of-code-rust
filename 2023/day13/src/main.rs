use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn transpose(board: &[String]) -> Vec<String> {
    let char_matrix: Vec<Vec<char>> = board.iter().map(|s| s.chars().collect()).collect();
    let res = (0..char_matrix[0].len())
        .map(|i| {
            char_matrix
                .iter()
                .map(|row| row.get(i).unwrap())
                .rev()
                .collect::<String>()
        })
        .collect();
    res
}

fn diff(s1: &str, s2: &str) -> u8 {
    let mut count_diff = 0u8;
    for (c1, c2) in s1.chars().zip(s2.chars()) {
        if c1 != c2 {
            count_diff += 1
        }
    }
    count_diff
}

fn is_mirror(board: &[String], mux: usize) -> usize {
    let rows = board.len();
    for i in 0..rows - 1 {
        let mut start = i;
        let mut end = i + 1;
        let mut smudges = 0u8;
        while board[start] == board[end] || smudges <= 1 {
            let d = diff(&board[start], &board[end]);
            if d > 0 {
                smudges += d;
            }
            end += 1;
            start = start.checked_sub(1).unwrap_or(end + 1);
            if start > end || end == rows {
                if smudges == 1 {
                    return (i + 1) * mux;
                } else {
                    break;
                }
            }
        }
    }
    0
}

fn find_mirror(board: &[String]) -> usize {
    let res = is_mirror(board, 100);
    if res > 0 {
        res
    } else {
        is_mirror(&transpose(board), 1)
    }
}

fn show_board(board: &[String]) {
    for b in board {
        println!("{b}");
    }
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mut lines: Vec<String> = reader.lines().map(|s| s.unwrap()).collect();
    lines.push(String::new());

    let mut board: Vec<String> = Vec::new();
    let mut sum = 0;
    for l in &lines[1..] {
        if l.is_empty() {
            let tmp = find_mirror(&board);
            sum += tmp;
            board = Vec::new();
            continue;
        }

        board.push(l.to_string());
    }
    println!("Total: {sum}");
    // println!("Hello, world!");
}
