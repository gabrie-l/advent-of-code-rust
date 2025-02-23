use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::ops::Deref;
use std::usize;

fn show(p: &[String]) {
    for line in p {
        println!("{}", line);
    }
    println!();
}

fn turn(p: &[String], dir: &str) -> Vec<String> {
    let range: Vec<usize> = match dir {
        "right" => (0..p[0].len()).collect(),
        _ => (0..p[0].len()).rev().collect(),
    };
    let mut res: Vec<String> = Vec::new();
    for i in range {
        res.push(p.iter().map(|row| row.chars().nth(i).unwrap()).collect())
    }
    let res = match dir {
        "right" => res.iter_mut().map(|c| c.chars().rev().collect()).collect(),
        _ => res.iter_mut().map(|c| c.chars().collect()).collect(),
    };
    res
}

fn roll_rocks(platform: &mut [String], dir: &str) -> Vec<String> {
    platform
        .iter_mut()
        .map(|item| {
            item.split('#')
                .map(|part| {
                    let mut sorted: Vec<char> = part.chars().collect();
                    sorted.sort();
                    match dir {
                        "right" => sorted.sort(),
                        _ => sorted.sort_unstable_by(|a, b| b.cmp(a)),
                    }
                    sorted.into_iter().collect()
                })
                .collect::<Vec<String>>()
                .join("#")
        })
        .collect()
}

fn cycle(platform: &[String]) -> Vec<String> {
    let mut north = turn(platform, "right");
    north = roll_rocks(&mut north, "right");
    north = turn(&north, "left");
    let west: Vec<String> = roll_rocks(&mut north, "left");
    let mut south = turn(&west, "right");
    south = roll_rocks(&mut south, "left");
    south = turn(&south, "left");
    let east: Vec<String> = roll_rocks(&mut south, "right");
    east
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mut platform: Vec<String> = reader
        .lines()
        .map(|l| l.unwrap().chars().collect())
        .collect();
    show(&platform);
    let mut grids: HashSet<Vec<String>> = HashSet::new();
    let mut grid_vec: Vec<Vec<String>> = Vec::new();
    let mut iterations: usize = 0;

    grids.insert(platform.clone());
    grid_vec.push(platform.clone());
    loop {
        platform = cycle(&platform);
        iterations += 1;
        if !grids.insert(platform.clone()) {
            break;
        }
        grid_vec.push(platform.clone());
    }
    let total_cycles: usize = 1_000_000_000;
    let first: usize = grid_vec.iter().position(|x| *x == platform).unwrap();
    let final_state: usize = (total_cycles - first) % (iterations - first) + first;

    println!("iterations: {iterations}");
    // show(&platform);
    let mut total: usize = 0;
    grid_vec[final_state]
        .iter()
        .enumerate()
        .for_each(|(idx, row)| {
            row.chars().for_each(|c| {
                if c == 'O' {
                    total += grid_vec[final_state].len() - idx
                }
            })
        });
    println!("Total: {total}");
}
