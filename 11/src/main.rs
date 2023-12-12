use std::fs;

// empty row turn into this many lines
static GAP: i32 = 1000000;

fn main() {
    part_one();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn build_galaxy(path: &str) -> (Vec<Vec<char>>, Vec<usize>, Vec<usize>) {
    let content = read_file(path);
    let mut galaxy: Vec<Vec<char>> = Vec::new();
    let mut open_rows: Vec<usize> = Vec::new();
    let mut open_cols_counts: Vec<usize> = Vec::new();
    
    let lines: Vec<&str> = content.lines().collect();
    for j in 0..lines.len() {
        let line = lines.get(j).unwrap();
        if open_cols_counts.len() == 0 {
            for _ in 0..line.len() {
                open_cols_counts.push(0);
            }
        }

        let chars: Vec<char> = line.chars().collect();
        let mut new_row: Vec<char> = Vec::new();
        let mut contains_no_galaxy = true;
        for i in 0..chars.len() {
            let char = *chars.get(i).unwrap();
            new_row.push(char);
            if char == '#' {
                open_cols_counts[i] += 1;
                contains_no_galaxy = false;
            }
        }

        if contains_no_galaxy {
            open_rows.push(j);
        }
        galaxy.push(new_row);
    }
    let mut open_cols: Vec<usize> = Vec::new();
    for i in 0..open_cols_counts.len() {
        if open_cols_counts.get(i).unwrap() == &0 {
            open_cols.push(i);
        }
    }

    println!("Open rows: {:?}", open_rows);
    println!("Open cols: {:?}", open_cols);
    ( galaxy, open_rows, open_cols )
}

/* PART TWO */
// fn part_two() {
// }

/* PART ONE */
fn part_one() {
    let galaxy_stuff: (Vec<Vec<char>>, Vec<usize>, Vec<usize>)  = build_galaxy("./input/input.txt");
    // let galaxy_stuff: (Vec<Vec<char>>, Vec<usize>, Vec<usize>)  = build_galaxy("./input/input2.txt");
    // Had to change how I do this stuff for part 2 so we're doing it like this even though it's bad :)
    let galaxy = galaxy_stuff.0;
    let open_rows = galaxy_stuff.1;
    let open_cols = galaxy_stuff.2;

    for i in 0..galaxy.len() {
        for j in 0..galaxy.get(i).unwrap().len() {
            print!("{}", galaxy.get(i).unwrap().get(j).unwrap());
        }
        println!();
    }

    let mut galaxies: Vec<(usize, usize)> = Vec::new();

    for i in 0..galaxy.len() {
        let row = galaxy.get(i).unwrap();
        for j in 0..row.len() {
            if row.get(j).unwrap() == &'#' {
                galaxies.push((j,i));
            }
        }
    }

    println!("Galaxies: {:?}", galaxies.len());
    println!("Galaxies: {:?}", galaxies);
    let mut total_distance: i128 = 0;
    for i in 0..galaxies.len() {
        let start_galaxy = galaxies.get(i).unwrap();
        for j in i+1..galaxies.len() {
            let end_galaxy = galaxies.get(j).unwrap();
            let mut distance = (start_galaxy.0 as i32 - end_galaxy.0 as i32).abs() + (start_galaxy.1 as i32 - end_galaxy.1 as i32).abs();
            for x in &open_cols {
                if (x > &start_galaxy.0 && x < &end_galaxy.0) || (x < &start_galaxy.0 && x > &end_galaxy.0) {
                    // distance already accounts for 1 in the gap
                    distance += GAP - 1;
                }
            }
            for y in &open_rows {
                if (y > &start_galaxy.1 && y < &end_galaxy.1) || (y < &start_galaxy.1 && y > &end_galaxy.1) {
                    // distance already accounts for 1 in the gap
                    distance += GAP - 1;
                }
            }
            println!("{:?} -> {:?} Distance: {}", i+1, j+1, distance);
            total_distance += distance as i128;
        }
    }

    println!("Total distance: {}", total_distance);

}
