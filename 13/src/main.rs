use std::fs;

fn main() {
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn find_reflection_line(pattern: &Vec<String>) -> (bool, usize) {

    for i in 0..pattern.len() {
        if i == pattern.len() - 1 {
            break;
        }

        let index_refletion = has_reflection(pattern, i);
        // println!("Has reflection: {} {}", index_refletion, i);
        if index_refletion {
            return (true, i);
        }
    }

    let rotated: Vec<String> = rotate_pattern(&pattern);

    for i in 0..rotated.len() {
        if i == rotated.len() - 1 {
            break;
        }

        let index_refletion = has_reflection(&rotated, i);
        // println!("Has reflection: {} {}", index_refletion, i);
        if index_refletion {
            return (false, i);
        }
    }

    (false, 100000_usize)
}

fn rotate_pattern(pattern: &Vec<String>) -> Vec<String> {
    let mut rotated_pattern: Vec<String> = Vec::new();
    for _ in 0..pattern.get(0).unwrap().len() {
        rotated_pattern.push(String::new());
    }

    for line in pattern {
        for rotate_index in 0..line.len() {
            rotated_pattern[rotate_index].push_str(&line[rotate_index..rotate_index + 1]);
        }
    }

    rotated_pattern
}

fn has_reflection(pattern: &Vec<String>, index: usize) -> bool {
    let mut contains_reflection = false;

    for i in 0..pattern.len() {
        if i > index || index + i + 1 >= pattern.len(){
            break;
        }
        contains_reflection = pattern[index - i] == pattern[index + i + 1];
        if !contains_reflection {
            break;
        }
    }

    contains_reflection
}

// (is horizontal, index)
fn find_reflection_line_smudge(pattern: &Vec<String>) -> (bool, usize) {
    // I LITERALLY DO THE SAME THING TWICE HERE LMAO
    for i in 0..pattern.len() {
        if i == pattern.len() - 1 {
            break;
        }

        let index_refletion = has_reflection_smudge(pattern, i);
        // println!("Has reflection: {} {}", index_refletion, i);
        if index_refletion {
            return (true, i);
        }
    }

    let rotated: Vec<String> = rotate_pattern(&pattern);

    for i in 0..rotated.len() {
        if i == rotated.len() - 1 {
            break;
        }

        let index_refletion = has_reflection_smudge(&rotated, i);
        // println!("Has reflection: {} {}", index_refletion, i);
        if index_refletion {
            return (false, i);
        }
    }

    (false, 100000_usize)
}

fn has_reflection_smudge(pattern: &Vec<String>, index: usize) -> bool {
    println!("Index: {}", index);
    let mut contains_reflection = false;
    let mut smudged = false;

    for i in 0..pattern.len() {
        if i > index || index + i + 1 >= pattern.len(){
            break;
        }
        let mut num_diff = 0;
        for j in 0..pattern[index-i].len() {
            if pattern[index-i].chars().nth(j).unwrap() != pattern[index+i+1].chars().nth(j).unwrap() {
                num_diff += 1;
            }
        }
        if num_diff == 1 && !smudged{
            smudged = true;
            contains_reflection = true;
        } else if num_diff == 0 {
            contains_reflection = true;
        } else {
            contains_reflection = false;
            break;
        }
    }

    smudged && contains_reflection
}

/* PART TWO */
fn part_two() {
    let content = read_file("input/input.txt");

    let mut horizontal_pattern: Vec<String> = Vec::new();
    let mut total: i32 = 0;

    for line in content.lines() {
        if line == "" {
            let reflection = find_reflection_line_smudge(&horizontal_pattern);
            println!("Reflection: {:?}", reflection);
            if(reflection.1 == 100000_usize) {
                for line in &horizontal_pattern {
                    println!("ERROR: {}", line);
                }
            }
            if reflection.0 {
                total += (reflection.1 as i32 + 1) * 100;
            } else {
                total += reflection.1 as i32 + 1;
            }
            horizontal_pattern.clear();
        } else {
            horizontal_pattern.push(line.to_string());
        }
    }

    println!("Total: {}", total);
}

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");

    let mut horizontal_pattern: Vec<String> = Vec::new();
    let mut total: i32 = 0;

    for line in content.lines() {
        if line == "" {
            let reflection = find_reflection_line(&horizontal_pattern);
            println!("Reflection: {:?}", reflection);
            if reflection.0 {
                total += (reflection.1 as i32 + 1) * 100;
            } else {
                total += reflection.1 as i32 + 1;
            }
            horizontal_pattern.clear();
        } else {
            horizontal_pattern.push(line.to_string());
        }
    }

    println!("Total: {}", total);
}
