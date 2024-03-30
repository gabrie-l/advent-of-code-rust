use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_location(seed: i64, map: &Vec<Vec<Vec<i64>>>) -> i64 {
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

fn remap(lo: i64, hi: i64, m: &Vec<Vec<i64>>) -> Vec<Vec<i64>> {
    let mut ans = vec![];

    for vec in m {
        let (dst, src, r) = (vec[0], vec[1], vec[2]);
        let end = src + r - 1;
        let d = dst - src;

        if !(end < lo || src > hi) {
            ans.push(vec![src.max(lo), end.min(hi), d]);
        }
    }

    let mut result: Vec<Vec<i64>> = vec![];
    for (idx, interval) in ans.iter().enumerate() {
        let (l, r, d) = (interval[0], interval[1], interval[2]);
        result.push(vec![l + d, r + d]);
        if idx < ans.len() - 1 && ans[idx + 1][0] > r + 1 {
            result.push(vec![r + 1, ans[idx + 1][0] - 1])
        }
    }
    if ans.len() == 0 {
        result.push(vec![lo, hi]);
        return result;
    }
    if ans[0][0] != lo {
        result.push(vec![lo, ans[0][0] - 1]);
    }
    if ans.last_mut().unwrap()[1] != hi {
        result.push(vec![ans.last_mut().unwrap()[1] + 1, hi])
    }

    result
}

fn main() {
    let file = File::open("input").expect("File missing");
    let reader = BufReader::new(file);
    let seed_regex = Regex::new(r"seeds: (\d+(?: \d+)*)").unwrap();
    let name_regex = Regex::new(r"\w+-to-\w+ map:+").unwrap();
    let map_regex = Regex::new(r"\d+ \d+ \d+").unwrap();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    // Match against the first line
    let seeds = seed_regex
        .captures(&lines[0])
        .and_then(|seed_match| seed_match.get(1))
        .map_or(Vec::new(), |s| {
            s.as_str()
                .split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        });

    let mut seed_intervals: Vec<Vec<i64>> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        seed_intervals.push(vec![seeds[i], seeds[i + 1]]);
    }

    let mut maps: Vec<Vec<Vec<i64>>> = vec![];
    for line in &lines[2..] {
        match line {
            l if name_regex.captures_iter(l).next().is_some() => {
                maps.push(vec![]);
                continue;
            }
            l if map_regex.captures_iter(l).next().is_some() => {
                let map_values: Vec<i64> = l
                    .split_whitespace()
                    .map(|num| num.parse().unwrap())
                    .collect();
                maps.last_mut().expect("Error no index").push(map_values);
            }
            _ => {
                maps.last_mut()
                    .expect("Error no index")
                    .sort_by(|a, b| a[1].cmp(&b[1]));
            }
        }
    }

    let mut ans = i64::MAX;
    for s in seed_intervals {
        let mut og_intervals: Vec<Vec<i64>> = vec![vec![s[0], s[0] + s[1] - 1]];
        let mut new_intervals: Vec<Vec<i64>> = vec![];
        for m in &maps {
            for int in og_intervals {
                let lo = int[0];
                let hi = int[1];
                for new_int in remap(lo, hi, &m) {
                    new_intervals.push(new_int);
                }
            }
            (og_intervals, new_intervals) = (new_intervals, vec![]);
        }
        for int in og_intervals {
            let lo = int[0];
            ans = ans.min(lo);
        }
    }
    println!("answer: {ans}");
}
