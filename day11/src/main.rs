use std::char;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

fn show_universe(board: &Vec<Vec<char>>) {
    for b in board {
        println!("{:?}", b);
    }
}

fn expand(universe: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut idx: usize = 0;
    loop {
        if idx == universe.len() {
            break;
        }
        if !universe[idx].contains(&'#') {
            universe.insert(idx, vec!['.'; universe[0].len()]);
            idx += 1;
        }
        idx += 1;
    }
    //expand universe colums
    let mut col: usize = 0;
    'col_loop: loop {
        if col >= universe[0].len() - 1 {
            break;
        }
        for row in 0..universe.len() {
            if universe[row][col] == '#' {
                col += 1;
                continue 'col_loop;
            }
            //duplicate col
        }
        for row in 0..universe.len() {
            universe[row].insert(col, '.');
        }
        col += 2;
    }
    universe.to_owned()
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mut lines: Vec<Vec<char>> = reader
        .lines()
        .map(|s| s.unwrap().chars().collect())
        .collect();

    //expand universe rows
    let universe = expand(&mut lines);
    show_universe(&universe);
    let mut coords: Vec<Vec<usize>> = Vec::new();

    for row in 0..universe.len() {
        for col in 0..universe[0].len() {
            if universe[row][col] == '#' {
                //getting galaxy coordinates
                coords.push(vec![row, col]);
            }
        }
    }

    for i in 0..coords.len() {
        println!("{:?}", coords[i].len());
    }
    let mut dists: Vec<Vec<usize>> = vec![vec![0usize]; coords.len()];
    let mut pairs: HashSet<(usize, usize)> = HashSet::new();
    for i in 0..coords.len() {
        for j in 1..coords.len() {
            let idx = (i + j) % coords.len();
            if !pairs.contains(&(i.min(idx), i.max(idx))) {
                let x = coords[i][0].abs_diff(coords[idx][0]);
                let y = coords[i][1].abs_diff(coords[idx][1]);
                dists[i].push(x + y);
                pairs.insert((i.min(idx), i.max(idx)));
            }
        }
    }
    let total: usize = dists.iter().flatten().sum();
    println!("{:?}", total);
    // println!("rows: {}", lines.len());
    // println!("cols: {}", lines[0].len());
}
