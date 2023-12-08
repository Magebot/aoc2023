use std::fs;

fn main() {
    part_one();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

/* PART TWO */
// fn part_two() {
// }

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");
    println!("{}", content);
}
