use std::error::Error;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;

fn smudge(str1: &str, str2: &str) -> Result<String, Box<dyn Error>> {
    //str1 is the string closest to the edge of the board
    let mut diff_idx = None;
    let mut diff_char = None;
    for (idx, (c1, c2)) in str1.chars().zip(str2.chars()).enumerate() {
        if c1 != c2 {
            if diff_idx.is_some() {
                return Err("More than one character difference".into()); // more than one char different
            }

            diff_idx = Some(idx);
            diff_char = Some(c1)
        }
    }
    let idx = diff_idx.unwrap();
    let mut result = String::new();
    for (i, c) in str1.chars().enumerate() {
        if i == idx {
            let switch = match diff_char.unwrap() {
                '#' => '.',
                _ => '#',
            };
            result.push(switch);
        } else {
            result.push(c);
        }
    }
    Ok(result)
}

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

fn is_mirror(board: &[String], mux: usize) -> usize {
    let rows = board.len();
    for i in 0..rows - 1 {
        let mut start = i;
        let mut end = i + 1;
        while board[start] == board[end] {
            end += 1;
            start = start.checked_sub(1).unwrap_or(end + 1);
            if start > end || end == rows {
                show_board(board);
                return (i + 1) * mux;
            }
        }
    }
    0
}

fn find_mirror(board: &[String]) -> usize {
    let res = is_mirror(board, 100);
    if res > 0 {
        println!("res: {res}");
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
            println!("TMP: {tmp}\n");
            board = Vec::new();
            continue;
        }

        board.push(l.to_string());
    }
    println!("Total: {sum}");
    // println!("Hello, world!");
}
