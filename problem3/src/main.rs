use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, usize};

fn get_number<'a>(idx: (usize, usize), matrix: &'a mut Vec<Vec<char>>) -> String {
    //check if the index is not valid
    if idx.0 >= matrix.len()
        || idx.1 >= matrix[0].len()
        || matrix[idx.0][idx.1] == 'X'
        || matrix[idx.0][idx.1] == '.'
    {
        return "".to_string();
    };

    let number = matrix[idx.0][idx.1].to_string();
    matrix[idx.0][idx.1] = 'X';
    let mut left: String = String::new();
    let mut right: String = String::new();
    if idx.1 > 0 {
        left = get_number((idx.0, idx.1 - 1), matrix)
    }
    if idx.1 < matrix[0].len() {
        right = get_number((idx.0, idx.1 + 1), matrix)
    }
    format!("{}{}{}", left, number, right)
}

fn check_surroundings(idx: (usize, usize), matrix: &mut Vec<Vec<char>>) {
    // let row_len = matrix.len();
    // let col_len = matrix[0].len();
    let is_safe = |(x, y): (usize, usize)| -> bool { x > 0 && y > 0 };
    if is_safe(idx) {
        println!("{:?}", get_number((idx.0 - 1, idx.1), matrix));
        println!("{:?}", get_number((idx.0, idx.1 - 1), matrix));
        println!("{:?}", get_number((idx.0 - 1, idx.1 - 1), matrix));
        println!("{:?}", get_number((idx.0 + 1, idx.1 - 1), matrix));
        println!("{:?}", get_number((idx.0 - 1, idx.1 + 1), matrix));
    }
    println!("{:?}", get_number((idx.0 + 1, idx.1), matrix));
    println!("{:?}", get_number((idx.0, idx.1 + 1), matrix));
    println!("{:?}", get_number((idx.0 + 1, idx.1 + 1), matrix));
}

fn part1(reader: BufReader<File>) {
    let symbols = vec!['*', '&', '$', '#', '@', '+', '/', '=', '-'];
    let mut chars: Vec<Vec<char>> = Vec::new();
    for line in reader.lines() {
        let text = match line {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Error reading line {e}");
                String::new()
            }
        };

        let tmp_vec: Vec<char> = text.chars().collect();
        chars.push(tmp_vec);
    }

    let mut mut_chars = chars.clone();
    for (row_idx, row) in chars.iter().enumerate() {
        for (col_idx, _col) in row.iter().enumerate() {
            if symbols.contains(&mut mut_chars[row_idx][col_idx]) {
                mut_chars[row_idx][col_idx] = 'X';
                check_surroundings((row_idx, col_idx), &mut mut_chars);
            }
        }
    }
}

fn main() {
    let file = match File::open("example.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error {e}");
            return;
        }
    };
    let reader = BufReader::new(file);
    part1(reader);
}
