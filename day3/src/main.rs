use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, u32, usize};

fn get_number<'a>(idx: (usize, usize), matrix: &'a mut Vec<Vec<char>>) -> String {
    if idx.0 >= matrix.len()
        || idx.1 >= matrix[0].len()
        || matrix[idx.0][idx.1] == 'X'
        || matrix[idx.0][idx.1] == '.'
    {
        return "".to_string();
    };

    let number = matrix[idx.0][idx.1];
    matrix[idx.0][idx.1] = 'X';
    let mut left: String = String::new();
    let mut right: String = String::new();
    if idx.1 > 0 {
        left = get_number((idx.0, idx.1 - 1), matrix)
    }
    if idx.1 < matrix[0].len() - 1 {
        right = get_number((idx.0, idx.1 + 1), matrix)
    }
    format!("{}{}{}", left, number, right)
}

fn check_surroundings(idx: (usize, usize), matrix: &mut Vec<Vec<char>>) -> u32 {
    // let row_len = matrix.len();
    // let col_len = matrix[0].len();
    let is_safe = |(x, y): (usize, usize)| -> bool { x > 0 && y > 0 };
    let mut results: Vec<String> = Vec::new();
    if is_safe(idx) {
        results.push(get_number((idx.0, idx.1 - 1), matrix));
        results.push(get_number((idx.0 - 1, idx.1 - 1), matrix));
        results.push(get_number((idx.0 + 1, idx.1 - 1), matrix));
        results.push(get_number((idx.0 - 1, idx.1), matrix));
        results.push(get_number((idx.0 - 1, idx.1 + 1), matrix));
    }
    results.push(get_number((idx.0 + 1, idx.1), matrix));
    results.push(get_number((idx.0, idx.1 + 1), matrix));
    results.push(get_number((idx.0 + 1, idx.1 + 1), matrix));
    let mut sum: u32 = 0;
    let mut gear_part_counter: u8 = 0;
    let mut gear_ratio: u32 = 1;
    for number_string in results.iter() {
        if let Ok(n) = number_string.parse::<u32>() {
            gear_ratio *= n;
            // commented for part 2
            // sum += n;
            gear_part_counter += 1
        }
    }
    if gear_part_counter == 2 {
        sum += gear_ratio
    }
    println!("{sum}");
    sum
}

fn part1(reader: BufReader<File>) {
    let _symbols = vec!['%', '*', '&', '$', '#', '@', '+', '/', '=', '-'];
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

    let mut total_num_sum: u32 = 0;
    let mut mut_chars = chars.clone();
    for (row_idx, row) in chars.iter().enumerate() {
        for (col_idx, _col) in row.iter().enumerate() {
            // if symbols.contains(&mut mut_chars[row_idx][col_idx]) {
            if mut_chars[row_idx][col_idx] == '*' {
                mut_chars[row_idx][col_idx] = 'X';
                total_num_sum += check_surroundings((row_idx, col_idx), &mut mut_chars);
            }
        }
    }
    println!("this is the total sum of the numbers: {}", total_num_sum);
}

fn main() {
    let file = match File::open("input.txt") {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error {e}");
            return;
        }
    };
    let reader = BufReader::new(file);
    part1(reader);
}
