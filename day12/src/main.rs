use std::fs::File;
use std::io::{BufRead, BufReader};

fn combinations(
    map: &[char],
    seq: &[usize],
    map_idx: usize,
    seq_idx: usize,
    dp: &mut [Vec<i64>],
) -> i64 {
    if map_idx >= map.len() {
        return if seq_idx == seq.len() { 1 } else { 0 };
    }
    if seq_idx == seq.len() {
        return if !map[map_idx..].contains(&'#') { 1 } else { 0 };
    }
    if dp[map_idx][seq_idx] >= 0 {
        // println!("got here");
        return dp[map_idx][seq_idx];
    }
    let mut result = 0;
    let case_1 = ['.', '?'];
    let case_2 = ['#', '?'];

    if case_1.contains(&map[map_idx]) {
        result += combinations(map, seq, map_idx + 1, seq_idx, dp);
    }
    if case_2.contains(&map[map_idx]) {
        let idx = seq[seq_idx] + map_idx;
        if idx <= map.len()
            && !map[map_idx..idx].contains(&'.')
            && (idx == map.len() || map[idx] != '#')
        {
            result += combinations(map, seq, idx + 1, seq_idx + 1, dp);
        }
    }
    dp[map_idx][seq_idx] = result;
    result
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);

    let mut total: i64 = 0;
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    for l in lines {
        let mut iter = l.split_whitespace();
        let mut map = iter.next().unwrap().to_owned();
        let mut sequence: Vec<usize> = iter
            .next()
            .unwrap()
            .split(',')
            .map(|n| n.parse().unwrap())
            .collect();
        println!("Sequence: {:?}", sequence);
        println!("Map: {}", map);
        //unfold the map
        let tmp_map = map.clone();
        let tmp_seq = sequence.clone();
        for _ in 0..4 {
            map += &("?".to_owned() + &tmp_map);
            sequence.extend(tmp_seq.iter());
        }
        let map: Vec<char> = map.chars().collect();
        let mut dp = vec![vec![-1i64; sequence.len() + 1]; map.len() + 1];
        let result = combinations(&map, &sequence, 0, 0, &mut dp);
        println!("Result: {result}");
        total += result;
    }
    println!("Result: {total}");
}
