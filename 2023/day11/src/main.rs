use std::char;
use std::collections::HashSet;
use std::env;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

fn show_universe(board: &Vec<Vec<char>>) {
    for b in board {
        println!("{:?}", b);
    }
}

fn extra_distance(
    p_1: &Vec<usize>,
    p_2: &Vec<usize>,
    galaxy_rows: &HashSet<usize>,
    galaxy_cols: &HashSet<usize>,
    coef: usize,
) -> usize {
    let mut total: usize = 0;
    let (x_1, y_1): (usize, usize) = (p_1[0], p_1[1]);
    let (x_2, y_2): (usize, usize) = (p_2[0], p_2[1]);
    for row in x_1.min(x_2)..x_1.max(x_2) {
        if !galaxy_rows.contains(&row) {
            total += coef;
        }
    }
    for col in y_1.min(y_2)..y_1.max(y_2) {
        if !galaxy_cols.contains(&col) {
            total += coef;
        }
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = File::open("input").expect("File not found");
    let coef: usize = args[1].parse::<usize>().unwrap() - 1;
    let reader = BufReader::new(f);
    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    // show_universe(&lines);
    let mut galaxy_rows: HashSet<usize> = HashSet::new();
    let mut galaxy_cols: HashSet<usize> = HashSet::new();
    let universe = lines;
    let mut coords: Vec<Vec<usize>> = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                //getting galaxy coordinates
                coords.push(vec![row, col]);
                galaxy_rows.insert(row);
                galaxy_cols.insert(col);
            }
        }
    }

    let mut dists: Vec<Vec<usize>> = vec![vec![0usize]; coords.len()];
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..coords.len() {
        for j in 1..coords.len() {
            let idx = (i + j) % coords.len();
            if !pairs.contains(&(i.min(idx), i.max(idx))) {
                let x = coords[i][0].abs_diff(coords[idx][0]);
                let y = coords[i][1].abs_diff(coords[idx][1]);
                let expansion_offset =
                    extra_distance(&coords[i], &coords[idx], &galaxy_rows, &galaxy_cols, coef);
                dists[i].push(x + y + expansion_offset);
                pairs.insert((i.min(idx), i.max(idx)));
            }
        }
    }
    let total: usize = dists.into_iter().flatten().sum();
    println!("{:?}", total);
    // println!("rows: {}", lines.len());
    // println!("cols: {}", lines[0].len());
}
