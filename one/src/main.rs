// use std::fs;

use std::fs;

fn main() {
    let file_path: &str = "input/input.txt";

    println!("input file: {}", file_path);

    let content = fs::read_to_string(file_path).expect("No content found!");
    // println!("Read file: {}", content);

    let mut calibration_values: Vec<u32> = Vec::new();
    for line in content.lines() {
        let mut iter = line.chars().skip_while(|ch| !ch.is_digit(10));
        let first = iter.next().unwrap().to_digit(10).unwrap();
        let mut iter = line.chars().rev().skip_while(|ch| !ch.is_digit(10));
        let second = iter.next().unwrap().to_digit(10).unwrap();

        calibration_values.push(first * 10 + second);
    }

    println!("{:?}", calibration_values);

    let sum: u32 = calibration_values.iter().sum();
    println!("{}", sum);
}
