use std::fs;

fn main() {
    solve();
}

static COL_LENGTH: usize = 140; // input 1
// static COL_LENGTH: usize = 11; // input 2
// static COL_LENGTH: usize = 20; // input 3

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn get_pipe_grid(path: &str) -> Vec<char> {
    let content = read_file(path);
    let mut pipes = Vec::new();
    for line in content.lines() {
        println!("{}", line);
        for c in line.chars() {
            pipes.push(c);
        }
    }

    pipes
}

fn get_char_at_pos(position: (usize, usize), grid: &Vec<char>) -> char {
    let rows = COL_LENGTH;
    grid[position.1 * rows + position.0]
}

fn solve() {
    let grid = get_pipe_grid("input/input.txt"); // 140x140
    // let grid = get_pipe_grid("input/input2.txt"); // 11x9
    // let grid = get_pipe_grid("input/input3.txt"); // 20x10

    let start_pos = grid.iter().position(|&x| x == 'S').unwrap();
    println!("STARTING AT {}", start_pos);
    let start_coords: (usize, usize) = (start_pos%COL_LENGTH, start_pos/COL_LENGTH);

    let mut path_grid: Vec<char> = grid.iter().map(|&_x| '.').collect();
    let mut steps = 0;
    let mut previous_move: usize = 3;
    let mut current_coords = start_coords;
    println!("STARTING AT {:?}", current_coords);
    while get_char_at_pos(current_coords, &grid) != 'S' || steps == 0 {
        let current_pos = get_char_at_pos(current_coords, &grid);

        if current_pos == 'S' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'N'; // (input 1)
            // path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'X'; // (input 2 & 3)
            // hard coding this because who cares
            // start with north move (input 1)
            previous_move = 2;
            current_coords = (
                current_coords.0,
                current_coords.1-1,
            );
            // start with east move (input 2 and 3)
            // previous_move = 3;
            // current_coords = (
            //     current_coords.0+1,
            //     current_coords.1,
            // );
        } else if current_pos == '|' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'N';
            if previous_move == 2 {
                previous_move = 2;
                current_coords = (
                    current_coords.0,
                    current_coords.1-1,
                )
            } else if previous_move == 0 {
                previous_move = 0;
                current_coords = (
                    current_coords.0,
                    current_coords.1+1,
                )
            }
        } else if current_pos == '-' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'X';
            if previous_move == 1 {
                previous_move = 1;
                current_coords = (
                    current_coords.0-1,
                    current_coords.1,
                )
            } else if previous_move == 3 {
                previous_move = 3;
                current_coords = (
                    current_coords.0+1,
                    current_coords.1,
                )
            }
        } else if current_pos == 'L' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'N';
            if previous_move == 0 {
                previous_move = 3;
                current_coords = (
                    current_coords.0+1,
                    current_coords.1,
                )
            } else if previous_move == 1 {
                previous_move = 2;
                current_coords = (
                    current_coords.0,
                    current_coords.1-1,
                )
            }
        } else if current_pos == 'J' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'N';
            if previous_move == 0 {
                previous_move = 1;
                current_coords = (
                    current_coords.0-1,
                    current_coords.1,
                )
            } else if previous_move == 3 {
                previous_move = 2;
                current_coords = (
                    current_coords.0,
                    current_coords.1-1,
                )
            }
        } else if current_pos == '7' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'X';
            if previous_move == 2 {
                previous_move = 1;
                current_coords = (
                    current_coords.0-1,
                    current_coords.1,
                )
            } else if previous_move == 3 {
                previous_move = 0;
                current_coords = (
                    current_coords.0,
                    current_coords.1+1,
                )
            }
        } else if current_pos == 'F' {
            path_grid[current_coords.1 * COL_LENGTH + current_coords.0] = 'X';
            if previous_move == 2 {
                previous_move = 3;
                current_coords = (
                    current_coords.0+1,
                    current_coords.1,
                )
            } else if previous_move == 1 {
                previous_move = 0;
                current_coords = (
                    current_coords.0,
                    current_coords.1+1,
                )
            }
        }
        steps += 1;
    }
    let farthest_dist = steps/2;
    println!("DONE {:?}", farthest_dist);

    // My approach was all wrong
    // Visual representation of what is happening: 
    // https://www.reddit.com/r/adventofcode/comments/18fgddy/2023_day_10_part_2_using_a_rendering_algorithm_to/
    // This could be done while the first loop is running, but I want a visual representation of what's happening so I'm doing it after the fact
    let mut in_loop = false;
    let mut in_count = 0;
    for i in 0..path_grid.len() {
        let current_coord = (i%COL_LENGTH, i/COL_LENGTH);
        let current_pos = get_char_at_pos(current_coord, &path_grid);
        if i % COL_LENGTH == 0 {
            println!();
            in_loop = false;
        }
        if current_pos == 'N' {
            in_loop = !in_loop;
        } else if in_loop  && current_pos != 'X' {
            path_grid[i] = 'I';
            in_count += 1;
        }
        print!("{}", path_grid[i]);
    }
    println!();
    println!("IN COUNT: {}", in_count)
}
