use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{char, str, usize};

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let instructions: Vec<char> = lines[0].chars().collect();
    println!("# of instructions: {}", instructions.len());
    for line in &lines[2..] {
        let elements: Vec<&str> = line.split('=').map(|s| s.trim()).collect::<Vec<&str>>();
        let key = elements[0];
        let val_pair: Vec<&str> = elements[1]
            .trim_matches(|c| c == '(' || c == ')')
            .split(',')
            .map(|s| s.trim())
            .collect();
        map.insert(key, val_pair);
    }

    let mut steps: usize = 0;
    let mut node: &str = "AAA";
    while node != "ZZZ" {
        let i = instructions[steps % instructions.len()];
        match i {
            'L' => {
                node = map.get(&node).unwrap()[0];
            }
            'R' => {
                node = map.get(&node).unwrap()[1];
            }
            _ => {
                println!("Invalid instruction");
                return;
            }
        }
        steps += 1;
    }
    println!("Total steps: {steps}");
}
