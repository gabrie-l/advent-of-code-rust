use maplit::hashmap;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str;
use std::u64;

fn forward_pass(params: Vec<u64>, work_map: &HashMap<&str, Vec<&str>>) -> Option<u64> {
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
                    let val = &split[0][2..].to_string().parse::<u64>().unwrap();
                    // println!("ITEM: {:?}", item);
                    // println!("DEST: {dest}");
                    // println!("p: {p} | val: {val}");
                    match split[0].chars().nth(1).unwrap() {
                        '<' => {
                            if p < val {
                                match dest {
                                    "A" => {
                                        println!("APPROVED");
                                        return Some(params.values().sum());
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
                                        return Some(params.values().sum());
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
                            return Some(params.values().sum());
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
}

fn range_pass<'a>(
    wf_name: &str,
    mut ranges: HashMap<&'a str, (u64, u64)>, // order: x:0-m:1-a:2-s:3
    workflows: &HashMap<&str, Vec<&'a str>>,
) -> u64 {
    match wf_name {
        "R" => return 0,
        "A" => {
            let mut result = 1;
            for (lo, hi) in ranges.values() {
                result *= (hi - lo) + 1
            }
            return result;
        }
        _ => {}
    }

    let curr_wf = workflows.get(wf_name).unwrap();
    let mut total = 0u64;
    for item in curr_wf {
        let split: Vec<&str> = item.split(':').collect();
        if split.len() == 2 {
            let dest = split[1];
            let p = split[0].get(0..1).unwrap();
            let number: u64 = split[0].get(2..).unwrap().parse().unwrap();
            let (lo, hi) = *ranges.get(p).unwrap();
            let op = if split[0].get(1..2).unwrap() == "<" {
                "<"
            } else {
                ">"
            };
            let t: (u64, u64);
            let f: (u64, u64);

            // println!("{:?}", number);
            let mut out_ranges = ranges.clone();
            if op == "<" {
                t = (lo, number - 1);
                f = (number, hi);
            } else {
                t = (number + 1, hi);
                f = (lo, number);
            }
            if t.0 <= t.1 {
                out_ranges.insert(p, t);
                total += range_pass(dest, out_ranges, workflows);
            }
            if f.0 <= f.1 {
                ranges.insert(p, f);
            } else {
                return 0;
            }
        } else {
            total += range_pass(item, ranges.clone(), workflows)
        }
    }

    total
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
    // let parts: Vec<Vec<u64>> = parts
    //     .iter()
    //     .map(|line| {
    //         re.find_iter(line)
    //             .map(|m| m.as_str().parse::<u64>().unwrap())
    //             .collect()
    //     })
    //     .collect();
    let mut work_map: HashMap<&str, Vec<&str>> = HashMap::new();
    for w in workflows.iter() {
        let splits: Vec<&str> = w.split('{').collect();
        let c: Vec<&str> = splits[1][..splits[1].len() - 1].split(',').collect();
        work_map.insert(splits[0], c);
    }

    //pass instructions
    let ranges = hashmap! {
        "x" => (1,4000),
        "m" => (1,4000),
        "a" => (1,4000),
        "s" => (1,4000)
    };
    let total = range_pass("in", ranges, &work_map);
    // while !parts.is_empty() {
    //     if let Some(v) = parts.pop() {
    //         if let Some(result) = forward_pass(v, &work_map) {
    //             println!("RESULT: {result}");
    //             total += result;
    //         };
    //     };
    // }
    println!("Sum of accepted parts: {total}");
}
