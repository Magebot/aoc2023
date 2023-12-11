use std::fs;

fn main() {
    let w_diffs = part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn parse_line(input: &str) -> Vec<i32> {
    input.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect()
}

fn get_diffs(input: &Vec<i32>) -> Vec<i32> {
    let mut diffs = Vec::new();
    for i in 0..input.len() - 1 {
        diffs.push(input[i + 1] - input[i]);
    }
    diffs
}

/* PART TWO */
fn part_two() -> Vec<i32>{
    let content: String = read_file("input/input.txt");
    let patterns: Vec<Vec<i32>> = content.lines().map(|x| parse_line(x)).collect::<Vec<Vec<i32>>>();

    let mut total = 0;
    let mut offsets: Vec<i32> = Vec::new();
    for pattern in patterns {
        let mut cur_pattern_seq: Vec<Vec<i32>> = Vec::new();
        cur_pattern_seq.push(pattern.clone());

        let mut all_zero = false;
        while !all_zero {
            let lastest = &cur_pattern_seq.iter().last().unwrap().clone();
            let diffs = get_diffs(lastest);
            cur_pattern_seq.push(diffs);
            all_zero = true;
            for aw in cur_pattern_seq.iter().last().unwrap() {
                if aw != &0 {
                    all_zero = false;
                }
            }
        }
        cur_pattern_seq.reverse();
        let mut offset = 0;
        for seq in cur_pattern_seq {
            // the only difference between part 1 and part 2 is this line
            offset = seq.iter().next().unwrap() - offset;
            // println!("{} | {:?}", offset, seq);
        }
        offsets.push(offset);
        total += offset;
    }

    println!("{}", total);
    offsets

}

/* PART ONE */
fn part_one() -> Vec<i32>{
    let content: String = read_file("input/input.txt");
    let patterns: Vec<Vec<i32>> = content.lines().map(|x| parse_line(x)).collect::<Vec<Vec<i32>>>();

    let mut total = 0;
    let mut offsets: Vec<i32> = Vec::new();
    for pattern in patterns {
        let mut cur_pattern_seq: Vec<Vec<i32>> = Vec::new();
        cur_pattern_seq.push(pattern.clone());

        let mut all_zero = false;
        while !all_zero {
            let lastest = &cur_pattern_seq.iter().last().unwrap().clone();
            let diffs = get_diffs(lastest);
            cur_pattern_seq.push(diffs);
            all_zero = true;
            for aw in cur_pattern_seq.iter().last().unwrap() {
                if aw != &0 {
                    all_zero = false;
                }
            }
        }
        cur_pattern_seq.reverse();
        let mut offset = 0;
        for seq in cur_pattern_seq {
            offset = seq.iter().last().unwrap() + offset;
            // println!("{:?} | {}", seq, offset);
        }
        offsets.push(offset);
        total += offset;
    }

    println!("{}", total);
    offsets

}
