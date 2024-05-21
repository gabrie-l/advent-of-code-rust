use maplit::hashmap;
use regex::Regex;
use regex::Split;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::u32;

fn forward_pass(params: Vec<u32>, work_map: &HashMap<&str, Vec<&str>>) -> Option<u32> {
    let mut stack = vec![work_map.get("in").unwrap()];
    println!("{:?}", &params);
    let params = hashmap! {
        'x' => params[0],
        'm' => params[1],
        'a' => params[2],
        's' => params[3],
    };
    let _first_wf = "in";
    while !stack.is_empty() {
        if let Some(s) = stack.pop() {
            for item in s {
                let split: Vec<&str> = item.split(':').collect();
                if split.len() == 2 {
                    let dest = split[1];
                    let p = params.get(&split[0].chars().next().unwrap()).unwrap();
                    let val = &split[0][2..].to_string().parse::<u32>().unwrap();
                    // println!("ITEM: {:?}", item);
                    // println!("DEST: {dest}");
                    // println!("p: {p} | val: {val}");
                    match split[0].chars().nth(1).unwrap() {
                        '<' => {
                            if p < val {
                                match dest {
                                    "A" => {
                                        println!("APPROVED");
                                        return Some(params.values().into_iter().sum());
                                    }
                                    "R" => {
                                        println!("REJECTED");
                                        return None;
                                    }
                                    _ => {
                                        stack.push(work_map.get(dest).unwrap());
                                        break;
                                    }
                                }
                            }
                        }
                        '>' => {
                            if p > val {
                                match dest {
                                    "A" => {
                                        println!("APPROVED");
                                        return Some(params.values().into_iter().sum());
                                    }
                                    "R" => {
                                        println!("REJECTED");
                                        return None;
                                    }
                                    _ => {
                                        stack.push(work_map.get(dest).unwrap());
                                        break;
                                    }
                                }
                            }
                        }
                        _ => unreachable!(),
                    };
                } else {
                    match split[0] {
                        "A" => {
                            println!("APPROVED");
                            return Some(params.values().into_iter().sum());
                        }
                        "R" => {
                            println!("REJECTED");
                            return None;
                        }
                        _ => {
                            stack.push(work_map.get(split[0]).unwrap());
                            break;
                        }
                    }
                }
            }
        }
    }
    None
    // while !stack.is_empty() {
    //     break;
    // }
}
fn main() {
    let f = File::open("input").expect("File not found");
    let reader: Vec<String> = BufReader::new(f).lines().map(|l| l.unwrap()).collect();
    let mut workflows: Vec<String> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    if let Some(idx) = reader.iter().position(|s| s.trim().is_empty()) {
        let (t1, t2) = reader.split_at(idx);
        workflows = t1.to_vec();
        parts = t2[1..].to_vec();
    }

    let re = Regex::new(r"([0-9]{1,4})+").unwrap();
    let mut parts: Vec<Vec<u32>> = parts
        .iter()
        .map(|line| {
            re.find_iter(line)
                .map(|m| m.as_str().parse::<u32>().unwrap())
                .collect()
        })
        .collect();
    let mut idx: usize;
    let mut work_map: HashMap<&str, Vec<&str>> = HashMap::new();
    let mut conditions: Vec<Vec<&str>> = Vec::new();
    for (i, w) in workflows.iter().enumerate() {
        let splits: Vec<&str> = w.split('{').collect();
        let c: Vec<&str> = splits[1][..splits[1].len() - 1].split(',').collect();
        work_map.insert(splits[0], c);
    }

    //pass instructions
    let mut total: u32 = 0;
    while !parts.is_empty() {
        if let Some(v) = parts.pop() {
            if let Some(result) = forward_pass(v, &work_map) {
                println!("RESULT: {result}");
                total += result;
            };
        };
    }
    println!("Sum of accepted parts: {total}");
}
