use std::{fs, collections::HashMap};

#[derive(Debug)]
struct Node {
    left: String,
    right: String,
}
fn main() {
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn create_instructions (line: &str) -> Vec<usize> {
    let steps: Vec<char> = line.chars().collect();
    let mut instructions: Vec<usize> = Vec::new();

    for step in steps {
        if step == 'L' {
            instructions.push(0);
        } else {
            instructions.push(1);
        }
    }

    instructions
}

fn create_node(line: &str) -> [String; 2] {
    let one = line.replace(")", "");
    let two = one.replace("(", "");
    let paths: Vec<&str> = two.split(",").collect();

    [paths[0].trim().to_string(),paths[1].trim().to_string()]
}

fn get_start_locations(map: &HashMap<String, [String;2]>) -> Vec<String> {
    let mut starts = Vec::new();

    for key in map.keys() {
        if key.chars().last().unwrap() == 'A' {
            starts.push(key.clone());
        }
    }

    starts

}

fn is_all_complete(locations: Vec<String>) -> bool {
    let mut all_complete = true;
    let mut num_z = 0;

    for loc in locations {
        if loc.chars().last().unwrap() != 'Z' {
            all_complete = false;
        } else {
            num_z += 1;
        }
    }
    println!("Z {}", num_z);
    all_complete
}

/* PART TWO */
fn part_two() {
    let content = read_file("input/input.txt");
    let lines: Vec<&str> = content.lines().collect();

    let instructions = create_instructions(lines[0]);

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for i in 2..lines.len() {
        let splits: Vec<&str> = lines[i].split("=").collect();
        map.insert(splits[0].trim().to_string(), create_node(splits[1].trim()));
    }
    
    let locations: Vec<String> = get_start_locations(&map);
    let mut steps_to_z: Vec<u64> = Vec::new();

    for i in 0..locations.len() {
        let mut current_loc = locations[i].clone();
        let mut steps = 0;
        let mut index = 0;
        while current_loc.chars().last().unwrap() != 'Z' {
            current_loc = map[&current_loc][instructions[index]].clone();
            steps += 1;
            index += 1;
            if index >= instructions.len() {
                index = 0;
            }
            println!("{:?}", locations);
        }
        steps_to_z.push(steps);
    }
    println!("{:?}", steps_to_z);
    let gcf = instructions.len() as u64;
    let mut lcm = ( steps_to_z.pop().unwrap() * steps_to_z.pop().unwrap() ) / gcf;
    while steps_to_z.len() > 0 {
        let next = steps_to_z.pop().unwrap();
        lcm = ( lcm * next ) / gcf;
    }

    println!("LCM: {}", lcm);

    // while !is_all_complete(locations.clone()) {
    //     steps += 1;

    //     for i in 0..locations.len() {
    //         locations[i] = map[&locations[i]][instructions[index]].clone();
    //     }

    //     index += 1;
    //     if index >= instructions.len() {
    //         index = 0;
    //     }

    //     println!("Steps: {} | Locations: {:?}", steps, locations);
    // }

}

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");
    let lines: Vec<&str> = content.lines().collect();

    let instructions = create_instructions(lines[0]);

    let mut map: HashMap<String, [String; 2]> = HashMap::new();
    for i in 2..lines.len() {
        let splits: Vec<&str> = lines[i].split("=").collect();
        map.insert(splits[0].trim().to_string(), create_node(splits[1].trim()));
    }

    let mut location = "AAA".to_string();
    let mut steps: u64 = 0;
    let mut index = 0;
    while location != "ZZZ".to_string() {
        location = map[&location][instructions[index]].clone();
        steps += 1;
        index += 1;
        if index >= instructions.len() {
            index = 0;
        }
        println!("Steps: {} | Location: {}", steps, location);
    }

    println!("Steps: {}", steps);

}
