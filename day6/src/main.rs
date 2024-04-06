use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();

    let time_regex = Regex::new(r"Time:\s+(\d+\s*)+").unwrap();
    let distance_regex = Regex::new(r"Distance:\s+(\d+\s*)+").unwrap();

    // part 1
    let times: Vec<u32> = time_regex
        .find_iter(&lines[0])
        .flat_map(|s| s.as_str().split_whitespace().filter_map(|d| d.parse().ok()))
        .collect();
    let distances: Vec<u32> = distance_regex
        .find_iter(&lines[1])
        .flat_map(|s| s.as_str().split_whitespace().filter_map(|d| d.parse().ok()))
        .collect();

    //part 2
    let time: u64 = times
        .iter()
        .fold(String::new(), |acc, &x| format!("{}{}", &acc, x))
        .parse()
        .unwrap();
    let distance: u64 = distances
        .iter()
        .fold(String::new(), |acc, &x| format!("{}{}", &acc, x))
        .parse()
        .unwrap();

    let mut final_answers: Vec<u64> = Vec::new();
    let mut results: Vec<u64> = Vec::new();
    for hold in 1..time {
        let dist_traveled: u64 = (time - hold) * hold;
        match dist_traveled {
            d if d > distance => results.push(hold),
            _ => continue,
        }
    }
    final_answers.push(results.len() as u64);
    println!(
        "Answer: {:?}",
        final_answers.iter().fold(1, |acc, &x| acc * x)
    );
}
