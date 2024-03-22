use aho_corasick::{AhoCorasick, PatternID};
use maplit::hashmap;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn modify_string(mymap: &HashMap<&str, &str>, mystring: String) -> String {
    let mut keys: Vec<&str> = vec![];
    for (key, _) in mymap.iter() {
        keys.push(key);
    }
    let ac = AhoCorasick::new(&keys).unwrap();
    let matches: Vec<PatternID> = ac
        .find_overlapping_iter(&mystring)
        .map(|mat| mat.pattern())
        .collect();
    let pattern: Vec<_> = matches.into_iter().map(|pat| pat.as_usize()).collect();
    let numbers: Vec<_> = pattern.into_iter().map(|num| mymap[keys[num]]).collect();
    let mut array_string = String::new();
    for &el in numbers.iter() {
        array_string.push_str(&el);
    }
    array_string
}

fn get_digit(mystring: &String) -> u32 {
    for (_i, c) in mystring.chars().enumerate() {
        if c.is_digit(10) {
            return c.to_digit(10).unwrap();
        }
    }
    return 0;
}

fn main() {
    // Open the file
    let file = File::open("input.txt");
    let numbers: HashMap<&str, &str> = hashmap! {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        "1" => "1",
        "2" => "2",
        "3" => "3",
        "4" => "4",
        "5" => "5",
        "6" => "6",
        "7" => "7",
        "8" => "8",
        "9" => "9",
    };
    // Check if the file was opened successfully
    let file = match file {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening file: {}", e);
            return;
        }
    };
    // Create a buffered reader
    let reader = BufReader::new(file);
    let mut total: i32 = 0;
    // Iterate over lines in the file
    for line in reader.lines() {
        // Check if line reading was successful
        let text = match line {
            Ok(line) => line,
            Err(_) => {
                eprintln!("Error reading line");
                return;
            }
        };
        println!("original string: {}", &text);
        //to solve part1 just don't modify the string
        let modified_string = modify_string(&numbers, text);
        let first = get_digit(&modified_string);
        let second = get_digit(&modified_string.chars().rev().collect());
        let tmp = format!("{}{}", first, second).parse::<i32>().unwrap();
        println!("{}", tmp);
        total += tmp;
    }
    println!("This is the answer: {}", total);
}
