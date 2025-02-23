use maplit::hashmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(reader: BufReader<File>) -> u32 {
    let cube_colors: HashMap<String, u8> = hashmap! {
        String::from("red") => 12,
        String::from("green") => 13,
        String::from("blue") => 14,
    };

    let mut wins: u32 = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                return 0;
            }
        };
        let game_split: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
        let game_number = match game_split[0].split_whitespace().nth(1) {
            Some(gn) => gn.parse::<u32>().unwrap_or(0),
            None => {
                return 0;
            }
        };

        let game = game_split[1].split(";").map(|s| s.to_string());
        let mut vic = true;
        'game_loop: for round in game {
            for (color, number) in cube_colors.iter() {
                if let Some(idx) = round.find(color) {
                    //checking if it is a two digit number

                    let value = match round
                        .chars()
                        .nth(idx - 3)
                        .expect("Not a digit")
                        .is_digit(10)
                    {
                        true => format!(
                            "{}{}",
                            round.chars().nth(idx - 3).unwrap(),
                            round.chars().nth(idx - 2).unwrap()
                        )
                        .parse::<u8>(),
                        false => round
                            .chars()
                            .nth(idx - 2)
                            .unwrap()
                            .to_string()
                            .parse::<u8>(),
                    };

                    match value {
                        Ok(parsed_value) => {
                            if parsed_value > *number {
                                // println!("Game {game_number} has {parsed_value} {color} cubes but only {number} available");
                                vic = false;
                                break 'game_loop;
                            }
                        }
                        Err(e) => {
                            eprintln!("Error while parsing {}", e);
                            return 0;
                        }
                    }
                };
            }
        }
        if vic {
            wins += game_number
        };
    }
    println!("Sum of games won: {wins}");
    wins
}
fn part2(reader: BufReader<File>) -> u32 {
    let mut sum: u32 = 0;
    for line in reader.lines() {
        let line = match line {
            Ok(l) => l,
            Err(_) => {
                return 0;
            }
        };
        let game_split: Vec<String> = line.split(":").map(|s| s.to_string()).collect();
        let colors = vec!["red", "green", "blue"];
        let game = game_split[1].split(";").map(|s| s.to_string());
        let mut cube_colors: HashMap<&str, u8> = hashmap! {
            "red" => 0,
            "green" => 0,
            "blue" => 0,
        };
        for round in game {
            for color in colors.iter() {
                if let Some(idx) = round.find(color) {
                    //checking if it is a two digit number

                    let value = match round
                        .chars()
                        .nth(idx - 3)
                        .expect("Not a digit")
                        .is_digit(10)
                    {
                        true => format!(
                            "{}{}",
                            round.chars().nth(idx - 3).unwrap(),
                            round.chars().nth(idx - 2).unwrap()
                        )
                        .parse::<u8>(),
                        false => round
                            .chars()
                            .nth(idx - 2)
                            .unwrap()
                            .to_string()
                            .parse::<u8>(),
                    };

                    match value {
                        Ok(parsed_value) => {
                            let number = cube_colors.get(color).unwrap();
                            if parsed_value > *number {
                                // println!("Game {game_number} has {parsed_value} {color} cubes but only {number} available");
                                cube_colors.insert(color, parsed_value);
                            }
                        }
                        Err(e) => {
                            eprintln!("Error while parsing {}", e);
                            return 0;
                        }
                    }
                };
            }
        }
        let mut tmp = 1u32;
        for (_, value) in cube_colors.iter() {
            tmp *= *value as u32;
        }
        sum += tmp;
    }
    println!("Sum of powers: {sum}");
    sum
}

fn main() {
    let file = File::open("input.txt");

    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("error opening file {}", e);
            return;
        }
    };
    let reader = BufReader::new(file);
    let _result_part2 = part2(reader);
    // let _result_part1 = part1(reader);
}
