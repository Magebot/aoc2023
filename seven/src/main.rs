use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
enum HandType {
    FIVE,
    FOUR,
    FULL,
    THREE,
    TPAIR,
    OPAIR,
    HIGH,
}

#[derive(Debug)]
struct Hand {
    hand_type: HandType,
    cards: String,
    bid: u64,
}

fn main() {
    part_one();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn get_hand_type(cards: String) -> HandType {
    let mut counts: HashMap<char, i32> = HashMap::new();
    for c in cards.chars() {
        let count = counts.entry(c).or_insert(0);
        *count += 1;
    }

    let counts_keys = counts.keys().len();
    if counts_keys == 1 {
        return HandType::FIVE;
    } else if counts_keys == 2 {
        for key in counts.keys() {
            if counts[key] == 4 {
                return HandType::FOUR;
            } else if counts[key] == 3 {
                return HandType::FULL;
            }
        }
    } else if counts_keys == 3 {
        for key in counts.keys() {
            if counts[key] == 3 {
                return HandType::THREE;
            } else if counts[key] == 2 {
                return HandType::TPAIR;
            }
        }
    } else if counts_keys == 4 {
        return HandType::OPAIR;
    }

    HandType::HIGH
}

fn build_hand(cards: String) -> Hand {
    let split: Vec<&str> = cards.split_whitespace().collect();
    let hand_type = get_hand_type(split[0].to_string());

    Hand {
        hand_type: hand_type,
        cards: String::from(split[0]),
        bid: split[1].parse::<u64>().unwrap(),
    }
}

/* PART TWO */
// fn part_two() {
// }

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");

    let mut hands: Vec<Hand> = Vec::new();
    for line in content.lines() {
        hands.push(build_hand(line.to_string()))
    }

    for hand in hands {
        println!("{:?}", hand);
    }

}
