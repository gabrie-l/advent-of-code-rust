use std::collections::HashMap;
use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::usize;

fn button_press(
    modules: &HashMap<String, (Vec<String>, char)>,
    memory: &mut HashMap<String, HashMap<String, bool>>,
    toggles: &mut HashMap<String, bool>,
    target_mods: &mut Vec<String>,
    cycles: &mut HashMap<String, usize>,
    seen: &mut HashMap<String, usize>,
    press_count: &mut usize,
) {
    // false -> low pulse | true -> high pulse
    // memory for flip-flop | false -> off, true -> on
    // memory for conjunction | false -> low pulse , true -> high pulse
    // queue has the module name, previous module and pulse state
    let mut tasks: VecDeque<(String, Option<String>, bool)> = VecDeque::new();
    tasks.push_back(("broadcaster".to_string(), None, false));
    let mut low_pulses = 1;
    let mut high_pulses = 0;
    while !tasks.is_empty() {
        let curr_task = tasks.pop_front().unwrap();
        let (module, prev, pulse) = curr_task;
        if !modules.contains_key(&module) {
            continue;
        }
        if module == "vr" && pulse {
            let origin = prev.as_ref().unwrap();
            let counter = seen.get_mut(origin).unwrap();
            *counter += 1;
            if !cycles.contains_key(origin) && target_mods.contains(origin) {
                let idx = target_mods.iter().position(|p| p == origin).unwrap();
                cycles.insert(target_mods.remove(idx).to_string(), *press_count);
            }
        }
        let (outputs, mod_type) = modules.get(&module).unwrap();
        match mod_type {
            '%' => {
                outputs.iter().for_each(|item| {
                    if modules.get(item).is_some() {
                        //if low (false) pulse
                        if !pulse {
                            let curr_toggle = if toggles.contains_key(&module) {
                                toggles.get(&module).unwrap()
                            } else {
                                toggles.insert(module.clone(), false);
                                &false
                            };
                            let out_pulse = !*curr_toggle;
                            if out_pulse {
                                high_pulses += 1
                            } else {
                                low_pulses += 1
                            }
                            tasks.push_back((item.to_string(), Some(module.clone()), out_pulse));
                        }
                    }
                });
                if !pulse {
                    let curr_toggle = toggles.get_mut(&module).unwrap();
                    *curr_toggle = !*curr_toggle;
                }
            }

            '&' => outputs.iter().for_each(|item| {
                let map = memory.get_mut(&module).unwrap();
                if let Some(p) = &prev {
                    map.insert(p.to_owned(), pulse);
                };
                let out_pulse = !map.values().copied().reduce(|a, b| a && b).unwrap_or(false);
                if out_pulse {
                    high_pulses += 1
                } else {
                    low_pulses += 1
                }
                tasks.push_back((item.to_string(), Some(module.clone()), out_pulse));
            }),
            'b' => outputs.iter().for_each(|item| {
                if modules.get(item).is_some() {
                    low_pulses += 1;
                    tasks.push_back((item.to_string(), Some(module.clone()), pulse))
                }
            }),
            _ => unreachable!(),
        }
    }
}

fn main() {
    let f = File::open("input").expect("File not found");
    let reader = BufReader::new(f);
    let mod_types = ['%', '&'];

    let mut modules: HashMap<String, (Vec<String>, char)> = HashMap::new();
    let mut memory: HashMap<String, HashMap<String, bool>> = HashMap::new();

    reader.lines().for_each(|l| {
        let line = l.expect("Could not read line");
        let first = line.chars().next().unwrap();
        let key = if mod_types.contains(&first) {
            line.split('-').next().unwrap().trim_end()[1..].to_string()
        } else {
            "broadcaster".to_string()
        };
        let values: Vec<String> = line
            .split('>')
            .nth(1)
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect();
        for val in &values {
            if memory.contains_key(val) {
                memory.get_mut(val).unwrap().insert(key.clone(), false);
            } else {
                let mut map: HashMap<String, bool> = HashMap::new();
                map.insert(key.to_string(), false);
                memory.insert(val.to_string(), map);
            }
        }
        modules.insert(key.clone(), (values, first));
    });
    let mut toggles: HashMap<String, bool> = HashMap::new();
    let mut spec_mods = vec![
        "dr".to_string(),
        "bm".to_string(),
        "tn".to_string(),
        "cl".to_string(),
    ];
    let mut seen: HashMap<String, usize> = HashMap::new();
    spec_mods.iter().for_each(|item| {
        seen.insert(item.clone(), 0);
    });

    let mut initial_map: HashMap<String, usize> = HashMap::new();
    let mut press_count = 1;
    while !spec_mods.is_empty() {
        button_press(
            &modules,
            &mut memory,
            &mut toggles,
            &mut spec_mods,
            &mut initial_map,
            &mut seen,
            &mut press_count,
        );
        press_count += 1;
    }
    println!("{:?}", initial_map);
}
