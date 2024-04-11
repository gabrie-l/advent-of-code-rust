use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn part1(hist_vec: Vec<Vec<i32>>) {
    let mut final_answer: Vec<i32> = Vec::new();
    for hist in hist_vec {
        let mut eval_hist = hist.clone();
        let mut length = hist.len();
        // let mut last_digits: Vec<i32> = Vec::new();
        let mut first_digits: Vec<i32> = Vec::new();
        loop {
            let mut tmp_hist: Vec<i32> = Vec::new();
            for idx in 0..length - 1 {
                tmp_hist.push(eval_hist[idx + 1] - eval_hist[idx])
            }
            // last_digits.push(*eval_hist.last().unwrap());
            first_digits.push(*eval_hist.first().unwrap());
            if tmp_hist
                .clone()
                .into_iter()
                .reduce(|a, b| a.abs() + b.abs())
                .unwrap()
                == 0
            {
                // println!("last digits: {:?}", last_digits);
                break;
            } else {
                length -= 1;
                eval_hist = tmp_hist.clone();
            }
        }
        let mut tmp = 0;
        for d in first_digits.into_iter().rev() {
            tmp = d - tmp;
        }
        println!("{:?}", &tmp);
        final_answer.push(tmp);
    }
    // println!("{:?}", &final_answer);
    println!("final_answers: {:?}", final_answer.into_iter().sum::<i32>());
}
fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let lines: Vec<_> = reader.lines().map(|s| s.unwrap()).collect();
    let numbers_regex = Regex::new(r"-?(\d+)").unwrap();
    let mut hist_vec: Vec<Vec<i32>> = Vec::new();
    for line in &lines {
        let numbers: Vec<i32> = numbers_regex
            .find_iter(line)
            .map(|s| s.as_str().parse().unwrap_or(0))
            .collect();
        println!("{:?}", numbers);
        hist_vec.push(numbers)
    }
    part1(hist_vec);
}
