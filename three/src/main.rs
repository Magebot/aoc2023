/* Oh sweet baby jesus that was annoying when you don't know rust */
use std::{fs, collections::{HashMap, hash_map::Entry}};

fn main() {
    this_is_pain();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn this_is_pain() {
    let input = read_file("input/input.txt");
    let mut grid = Vec::new();

    for line in input.lines() {
        grid.push(line);
    }

    let mut parts: Vec<u32> = Vec::new();
    let mut gear: (usize, usize) = (10000, 100000);
    let mut all_gears: HashMap<(usize, usize), Vec<u32>> = HashMap::new();
    let mut is_part_number = false;
    let mut is_gear = true;
    let mut cur_number = 0;
    
    let mut row = 0;
    while row < grid.len() { // row
        let mut col = 0;
        while col < grid[row].len() { // col
            let cur_char = grid[row].chars().nth(col).unwrap();

            if cur_char.is_digit(10) {
                cur_number = cur_number * 10 + cur_char.to_digit(10).unwrap();

                // Check surrounding
                // I can't have a negative usize so this is more annoying than it should be
                for yoff in [-1, 0, 1] {
                    for xoff in [-1, 0, 1] {
                        let mut checky = row;
                        let mut checkx = col;

                        // Skip if we're going out of bounds
                        if yoff < 0 && row == 0 { continue; }
                        if xoff < 0 && col == 0 { continue; }
                        if yoff > 0 && row == 139 { continue; }
                        if xoff > 0 && col == 139 { continue };

                        // Get the surrounding point
                        if yoff < 0 && row > 0 {
                            checky = row -1;
                        } else if yoff > 0 && row < grid.len() -1{
                            checky = row +1;
                        }

                        if xoff < 0 && col > 0{
                            checkx = col -1;
                        } else if xoff > 0 && col < grid[row].len() -1 {
                            checkx = col +1;
                        }
                        if !(yoff == 0 && xoff ==0) {
                            let cur_check = grid[checky].chars().nth(checkx).unwrap();
                            if cur_check != '.' && !cur_check.is_digit(10) {
                                is_part_number = true;
                                if(cur_check == '*' && is_gear) {
                                    is_gear = false;
                                    gear = (checky, checkx);
                                    println!("{} {} {}", cur_check, checky, checkx);
                                }
                            }
                        }
                    }
                }
            } else if cur_number > 0 {
                if is_part_number {
                    parts.push(cur_number);
                }
                if !is_gear {
                    match all_gears.entry(gear) {
                        Entry::Occupied(mut e) => {
                            e.get_mut().push(cur_number);
                        },
                        Entry::Vacant(v) => {
                            let mut temp = Vec::new();
                            temp.push(cur_number);
                            all_gears.insert(gear, temp);
                        }
                    }
                }
                is_part_number = false;
                is_gear = true;
                cur_number = 0;
            }

            col += 1;
        }
        row += 1;
    }

    let mut ratios = Vec::new();
    for (_, value) in all_gears {
        if value.len() > 1 {
            // We never see more than 2 here so we can just do this lazy
            // idk how you would have 3 gears attached in real life anyways
            let ratio = value[0] * value[1];
            ratios.push(ratio);
        }
    }

    let total: u32 = parts.iter().sum();
    let total_ratios: u32 = ratios.iter().sum();
    println!("{}", total); // Part 1 answer
    println!("{}", total_ratios); // Part 2 answer

}