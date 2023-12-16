use std::fs;

fn main() {
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn get_dish(content: String) -> Vec<Vec<char>> {
    let mut dish: Vec<Vec<char>> = Vec::new();

    for line in content.lines().rev() {
        if dish.len() == 0 {
            for _ in 0..line.len() {
                dish.push(Vec::new());
            }
        }

        for i in 0..line.len() {
            dish[i].push(line.chars().nth(i).unwrap());
        }
    }

    dish
}

fn rotate_counter(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut rotated_dish: Vec<Vec<char>> = Vec::new();

    for row in dish {
        if rotated_dish.len() == 0 {
            for _ in 0..row.len() {
                rotated_dish.push(Vec::new());
            }
        }

        for i in 0..row.len() {
            let cur_char = row.get(i).unwrap();
            if cur_char == &'O' {
                rotated_dish[i].insert(0, 'O');
            } else if cur_char == &'.' {
                rotated_dish[i].insert(0, '.');
            } else if cur_char == &'#' {
                rotated_dish[i].insert(0,'#');
            }
        }
    }

    rotated_dish
}

fn tilt(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut tilted_dish: Vec<Vec<char>> = Vec::new();

    for row in dish {
        let first = row;
        let mut tilted_vec: Vec<char> = Vec::new();

        for epoch in 0..first.len() {
            let index = first.len() - epoch - 1;
            if first.get(index).unwrap() == &'.' {
                tilted_vec.insert(0, '.');
            } else if first.get(index).unwrap() == &'O' {
                let mut insert_index = 0;
                while insert_index < tilted_vec.len() && tilted_vec.get(insert_index).unwrap() != &'#' {
                    insert_index += 1;
                }
                tilted_vec.insert(insert_index, 'O');
            } else if first.get(index).unwrap() == &'#' {
                tilted_vec.insert(0, '#');
            } 
        }

        tilted_dish.push(tilted_vec);
    }
    tilted_dish
}

fn cycle(dish: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let tilt_north = tilt(dish);
    let rotate_west = rotate_counter(&tilt_north);
    let tilt_west = tilt(&rotate_west);
    let rotate_south = rotate_counter(&tilt_west);
    let tilt_south = tilt(&rotate_south);
    let rotate_east = rotate_counter(&tilt_south);
    let tilt_east = tilt(&rotate_east);
    let rotate_north = rotate_counter(&tilt_east);
    rotate_north
}

fn get_load(dish: &Vec<Vec<char>>) -> i32 {
    let mut load = 0;
    for row in dish {
        let mut weight = 1;
        for seat in row {
            if seat == &'O' {
                load += weight;
            }
            weight += 1;
        }
    }
    load
}

/* PART TWO */
fn part_two() {
    let content = read_file("input/input.txt");
    let dish = get_dish(content);
    let mut loads: Vec<i32> = Vec::new();
    let mut cur_cycle = dish;

    for i in 0..1000 {
        cur_cycle = cycle(&cur_cycle);
        let load = get_load(&cur_cycle);
        loads.push(load);
    }

    println!("Loads: {:?}", loads);
}

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");
    let dish = get_dish(content);
    let mut tilted_dish: Vec<Vec<char>> = Vec::new();

    for row in dish {
        let first = row;
        let mut tilted_vec: Vec<char> = Vec::new();

        for epoch in 0..first.len() {
            let index = first.len() - epoch - 1;
            if first.get(index).unwrap() == &'.' {
                tilted_vec.insert(0, '.');
            } else if first.get(index).unwrap() == &'O' {
                let mut insert_index = 0;
                while insert_index < tilted_vec.len() && tilted_vec.get(insert_index).unwrap() != &'#' {
                    insert_index += 1;
                }
                tilted_vec.insert(insert_index, 'O');
            } else if first.get(index).unwrap() == &'#' {
                tilted_vec.insert(0, '#');
            } 
        }

        tilted_dish.push(tilted_vec);
    }

    let mut load = 0;
    for row in tilted_dish {
        let mut weight = 1;
        for seat in row {
            if seat == 'O' {
                load += weight;
            }
            weight += 1;
        }
    }
    println!("Load: {}", load);
}
