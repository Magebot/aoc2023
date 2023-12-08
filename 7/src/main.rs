use core::panic;
use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::fs;
use std::collections::HashMap;

#[derive(Debug)]
#[derive(PartialEq, Eq, Hash)]
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
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn get_hand_type(cards: String) -> HandType {
    // I believe the trick to part two is that you can just 
    // Add the J's to the count of the high non 'J' card unless it's JJJJJ
    let mut counts: HashMap<char, i32> = HashMap::new();
    let mut high_count = 0;
    let mut high_key = 'J';
    let mut jack_count = 0;
    for c in cards.chars() {
        if c == 'J' {
            jack_count += 1;
        } else {
            let count = counts.entry(c).or_insert(0);
            *count += 1;
            if *count > high_count {
                high_count = *count;
                high_key = c;
            }
        }

    }

    // the only thing this wouldn't be something else is if it's JJJJJ
    let count = counts.get(&high_key).unwrap_or(&5);
    counts.insert(high_key, count + jack_count);

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
fn convert_card_to_value(card: char) -> i32 {
    if card == 'A' {
        return 14;
    } else if card == 'K' {
        return 13;
    } else if card == 'Q' {
        return 12;
    } else if card == 'J' {
        return 1;
    } else if card == 'T' {
        return 10;
    }

    card.to_digit(10).unwrap() as i32
}

fn sort_by_cards(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        let a_split: Vec<char> = a.cards.chars().collect();
        let b_split: Vec<char> = b.cards.chars().collect();
        for i in 0..a_split.len() {
            if a_split[i] == b_split[i] {
                continue;
            }
            
            let a_card = convert_card_to_value(a_split[i]);
            let b_card = convert_card_to_value(b_split[i]);
            return a_card.cmp(&b_card);
        }
        Ordering::Equal
    });
}

fn convert_type_to_value(hand: &Hand) -> i32 {
    if hand.hand_type == HandType::FIVE {
        return 6;
    } else if hand.hand_type == HandType::FOUR {
        return 5;
    } else if hand.hand_type == HandType::FULL {
        return 4;
    } else if hand.hand_type == HandType::THREE {
        return 3;
    } else if hand.hand_type == HandType::TPAIR {
        return 2;
    } else if hand.hand_type == HandType::OPAIR {
        return 1;
    } 
    0
}

fn sort_by_hand(hands: &mut Vec<Hand>) {
    hands.sort_by(|a, b| {
        let a_hand_type = convert_type_to_value(a);
        let b_hand_type = convert_type_to_value(b);

        a_hand_type.cmp(&b_hand_type)
    })
}

fn part_two() {
    let content = read_file("input/input.txt");

    let mut hands: Vec<Hand> = Vec::new();
    for line in content.lines() {
        hands.push(build_hand(line.to_string()))
    }

    sort_by_cards(&mut hands);
    sort_by_hand(&mut hands);
    
    let mut total: u64 = 0;
    for i in 0..hands.len() {
        println!("{}: {:?}", i, hands[i]);
        total += hands[i].bid * ((i as u64) + 1_u64);
    }
    println!("Total: {}", total);
}

/* PART ONE */
// fn convert_card_to_value(card: char) -> i32 {
//     if card == 'A' {
//         return 14;
//     } else if card == 'K' {
//         return 13;
//     } else if card == 'Q' {
//         return 12;
//     } else if card == 'J' {
//         return 11;
//     } else if card == 'T' {
//         return 10;
//     }

//     card.to_digit(10).unwrap() as i32
// }

// fn sort_by_cards(hands: &mut Vec<Hand>) {
//     hands.sort_by(|a, b| {
//         let a_split: Vec<char> = a.cards.chars().collect();
//         let b_split: Vec<char> = b.cards.chars().collect();
//         for i in 0..a_split.len() {
//             if a_split[i] == b_split[i] {
//                 continue;
//             }
            
//             let a_card = convert_card_to_value(a_split[i]);
//             let b_card = convert_card_to_value(b_split[i]);
//             return a_card.cmp(&b_card);
//         }
//         Ordering::Equal
//     });
// }

// fn convert_type_to_value(hand: &Hand) -> i32 {
//     if hand.hand_type == HandType::FIVE {
//         return 6;
//     } else if hand.hand_type == HandType::FOUR {
//         return 5;
//     } else if hand.hand_type == HandType::FULL {
//         return 4;
//     } else if hand.hand_type == HandType::THREE {
//         return 3;
//     } else if hand.hand_type == HandType::TPAIR {
//         return 2;
//     } else if hand.hand_type == HandType::OPAIR {
//         return 1;
//     } 
//     0
// }

// fn sort_by_hand(hands: &mut Vec<Hand>) {
//     hands.sort_by(|a, b| {
//         let a_hand_type = convert_type_to_value(a);
//         let b_hand_type = convert_type_to_value(b);

//         a_hand_type.cmp(&b_hand_type)
//     })
// }

// fn part_one() {
//     let content = read_file("input/input.txt");

//     let mut hands: Vec<Hand> = Vec::new();
//     for line in content.lines() {
//         hands.push(build_hand(line.to_string()))
//     }

//     sort_by_cards(&mut hands);
//     sort_by_hand(&mut hands);
    
//     let mut total: u64 = 0;
//     for i in 0..hands.len() {
//         println!("{:?}", hands[i]);
//         total += hands[i].bid * ((i as u64) + 1_u64);
//     }
//     println!("Total: {}", total);
// }
