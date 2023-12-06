// use std::fs;

use std::fs;

fn main() {
    // part_one();
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

/* PART TWO */
fn part_two() {
    let file_path: &str = "input/input.txt";
    let content = read_file(file_path);

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in content.lines() {
        let digits_from_line: Vec<u32> = parse_line(line);
        
        // get first/last
        if digits_from_line.len() > 0 {
            let first = digits_from_line.first().unwrap();
            let second = digits_from_line.last().unwrap();
            calibration_values.push(first * 10 + second);
            println!("Results {:?} {}{}", digits_from_line, first, second);
        }
    }
    
    let total = get_sum_of_vec(calibration_values);
    println!("{}", total);
}

fn parse_line(line: &str) -> Vec<u32> {
    let mut digits = Vec::new();

    let mut i = 0;
    while i < line.len() {
        if line.chars().nth(i).unwrap().is_digit(10) {
            digits.push(line.chars().nth(i).unwrap().to_digit(10).unwrap());
        } else {
            let diff = line.len()-i;

            if diff > 4 {
                let temp = word_to_digit(&line[i..i+5]);
                match temp {
                    Some(x) => digits.push(x),
                    None => (),
                }
            }
            if diff > 3 {
                let temp = word_to_digit(&line[i..i+4]);
                match temp {
                    Some(x) => digits.push(x),
                    None => (),
                }
            }
            if diff > 2 {
                let temp = word_to_digit(&line[i..i+3]);
                match temp {
                    Some(x) => digits.push(x),
                    None => (),
                }
            }
        } 
        i+=1;
    }

    digits
}

fn word_to_digit(digitstr: &str) -> Option<u32> {
    let digit: Option<u32>;
    match digitstr {
        "one" => digit = Some(1),
        "two" => digit = Some(2),
        "three" => digit = Some(3),
        "four" => digit = Some(4),
        "five" => digit = Some(5),
        "six" => digit = Some(6),
        "seven" => digit = Some(7),
        "eight" => digit = Some(8),
        "nine" => digit = Some(9),
        _ => digit = None,
    }
    digit
}

fn get_sum_of_vec(values: Vec<u32>) -> u32 {
    values.iter().sum()
}

/* PART ONE */
fn part_one() {
    let file_path: &str = "input/input.txt";

    let content = read_file(file_path);

    let mut calibration_values: Vec<u32> = Vec::new();

    for line in content.lines() {
        calibration_values.push(read_calibration_digits(line));
    }


    let total = get_sum_of_vec(calibration_values);
    println!("{}", total);
}

fn read_calibration_digits(line: &str) -> u32 {

    let mut iter = line.chars().skip_while(|ch| !ch.is_digit(10));
    let first = iter.next().unwrap().to_digit(10).unwrap();
    let mut iter = line.chars().rev().skip_while(|ch| !ch.is_digit(10));
    let second = iter.next().unwrap().to_digit(10).unwrap();

    first * 10 + second
}