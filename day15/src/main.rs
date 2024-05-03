use std::collections::HashMap;

macro_rules! read_input {
    ($name:expr) => {{
        use std::fs::File;
        use std::io::BufRead;
        use std::io::BufReader;
        let f = File::open($name).expect("File not found");
        let reader = BufReader::new(f);
        let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
        let strings: Vec<String> = lines[0].trim().split(",").map(|s| s.to_string()).collect();
        strings
    }};
}

fn HASH(s: &str) -> usize {
    let mut current_value: u32 = 0;
    for c in s.chars() {
        let value = c as u32;
        current_value += value;
        current_value *= 17;
        current_value %= 256;
    }
    current_value as usize
}

fn main() {
    let strings: Vec<String> = read_input!("input");
    let mut boxes: Vec<HashMap<&str, usize>> = vec![HashMap::new(); 256];
    let mut lenses: Vec<Vec<&str>> = vec![Vec::new(); 256];
    let mut total = 0;
    strings.iter().for_each(|s| {
        if s.contains('=') {
            let split: Vec<&str> = s.split('=').collect();
            let label = split[0];
            let focal_length = split[1].parse::<usize>().unwrap();
            let box_num: usize = HASH(label);
            if boxes[box_num].contains_key(label) {
                boxes[box_num].insert(label, focal_length);
            } else {
                lenses[box_num].push(label);
                boxes[box_num].insert(label, focal_length);
            }
        } else {
            let split: Vec<&str> = s.split('-').collect();
            let label = split[0];
            let box_num: usize = HASH(label);
            if boxes[box_num].contains_key(label) {
                let position = lenses[box_num].iter().position(|x| *x == label).unwrap();
                lenses[box_num].remove(position);
                boxes[box_num].remove(label);
            }
        }
    });
    boxes.iter().enumerate().for_each(|(idx, map)| {
        lenses[idx].iter().enumerate().for_each(|(j, lbl)| {
            let power = (1 + idx) * (1 + j) * map.get(lbl).unwrap();
            total += power;
        })
    });
    // strings.iter().for_each(|s| total += HASH(s));

    println!("Total: {}", total);
    // println!("{:?}", convert_str(&lines[0]));
}
