use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_location(seed: u64, map: &Vec<Vec<Vec<u64>>>) -> u64 {
    let mut num = seed;
    for m in map {
        for vec in m {
            let (dest_start, source_start, length) = (vec[0], vec[1], vec[2]);
            if source_start <= num && num < source_start + length {
                num = dest_start + (num - source_start);
                break;
            }
        }
    }
    num
}

fn main() {
    let file = File::open("example").expect("File missing");
    let reader = BufReader::new(file);
    let seed_regex = Regex::new(r"seeds: (\d+ \d+ \d+ \d+)").unwrap();
    let name_regex = Regex::new(r"\w+-to-\w+:+").unwrap();
    let map_regex = Regex::new(r"\d+ \d+ \d+").unwrap();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Match against the first line
    let seeds = seed_regex
        .captures(&lines[0])
        .and_then(|seed_match| seed_match.get(1))
        .map_or(Vec::new(), |s| {
            s.as_str()
                .split_whitespace()
                .map(|x| x.parse::<u64>().unwrap())
                .collect()
        });

    let mut maps: Vec<Vec<Vec<u64>>> = vec![];
    for line in &lines[1..] {
        match line {
            l if name_regex.captures_iter(l).next().is_some() => {
                continue;
            }
            l if map_regex.captures_iter(l).next().is_some() => {
                let map_values: Vec<u64> = l
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                maps.last_mut().expect("Error no index").push(map_values);
            }
            _ => maps.push(vec![]),
        }
    }
    for seed in seeds {
        println!(
            "find location for seed #{}: {}",
            seed,
            find_location(seed.into(), &maps)
        )
    }
    println!("maps {:?}", maps);
}
