use maplit::{self, hashmap};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u32 {
    let mut total = 0;
    let mut card_map: HashMap<u32, u32> = (1..=6000).map(|k| (k, 1)).collect();
    let mut card_idx: u32 = 0;
    for line in reader.lines() {
        if let Ok(line) = line {
            card_idx += 1;
            let splits: Vec<&str> = line.split("|").collect();
            let (first_split, second_split) = (splits[0], splits[1]);
            let played_numbers: Vec<u32> = second_split
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            let winning_numbers: Vec<u32> = first_split
                .split(":")
                .nth(1)
                .unwrap_or_else(|| {
                    eprintln!("Error: Missing played numbers");
                    return "";
                })
                .split_whitespace()
                .map(|s| s.parse::<u32>().unwrap())
                .collect();
            //count winning numbers
            let mut counter: u32 = 0;
            for entry in played_numbers.iter() {
                if winning_numbers.contains(entry) {
                    print!("{} ", entry);
                    counter += 1;
                }
            }
            // let points = match counter {
            //     c if c > 1 => 2u32.pow(c - 1),
            //     1 => 1u32,
            //     _ => 0,
            // };
            // println!("Points for this round: {}", points);
            // total += points;
            let card_count: u32 = *card_map.get(&card_idx).unwrap_or(&0);
            if counter > 0 {
                for idx in 1..counter + 1 {
                    if let Some(value) = card_map.get_mut(&(card_idx + idx)) {
                        *value += card_count
                    }
                }
            }
            println!("number of #{} cards: {}", card_idx, card_count);
            total += card_count;
        };
    }
    total
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let f = match File::open(&args[1]) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error {}", e);
            return;
        }
    };

    let reader = BufReader::new(f);
    println!("Total points: {}", part1(reader))

    // println!("Hello, world!");
}
