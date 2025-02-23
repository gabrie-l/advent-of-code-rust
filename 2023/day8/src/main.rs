use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, str, usize};

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    // part 2
    let start_point_regex = Regex::new(r"..A").unwrap();
    let end_point_regex = Regex::new(r"..Z").unwrap();
    let mut start_points: Vec<&str> = Vec::new();

    let instructions: Vec<char> = lines[0].chars().collect();
    println!("# of instructions: {}", instructions.len());
    for line in &lines[2..] {
        let elements: Vec<&str> = line.split('=').map(|s| s.trim()).collect::<Vec<&str>>();
        let key = elements[0];
        if start_point_regex.is_match(key) {
            start_points.push(key)
        }
        let val_pair: Vec<&str> = elements[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|s| s.trim())
            .collect();
        map.insert(key, val_pair);
    }

    let mut lcm: Vec<u64> = Vec::new();
    for node in start_points {
        let mut steps: usize = 0;
        let mut next_node = node;
        while !end_point_regex.is_match(next_node) {
            let i = instructions[steps % instructions.len()];
            match i {
                'L' => {
                    next_node = map.get(next_node).unwrap()[0];
                }
                'R' => {
                    next_node = map.get(next_node).unwrap()[1];
                }
                _ => {
                    println!("Invalid instruction");
                    return;
                }
            }
            steps += 1;
        }
        lcm.push(steps as u64);
        println!("Steps for next_node {next_node}: {steps}");
    }
    let result = lcm.into_iter().reduce(|a, b| (a * b) / gcd(a, b)).unwrap();
    println!("result: {result}");
}
