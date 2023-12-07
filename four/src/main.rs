use std::fs;

#[derive(Debug)]
struct Card {
    winning_nums: Vec<i32>,
    my_nums: Vec<i32>,
    wins: u32,
    copies: u32,
}

fn main() {
    // part_one();
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn parse_line(line: &str) -> Card {

    let mut card: Card = Card { 
        winning_nums: Vec::new(),
        my_nums: Vec::new(),
        wins: 0,
        copies: 1,
    };

    let colon_loc = line.find(":").unwrap();
    let raw_numbers: &str = &line[colon_loc+1..];
    let pipe_loc = raw_numbers.find("|").unwrap();
    let raw_winning = &raw_numbers[..pipe_loc];
    let raw_mine = &raw_numbers[pipe_loc+1..];

    for num in raw_winning.split_whitespace() {
        card.winning_nums.push(num.parse::<i32>().unwrap());
    }

    for num in raw_mine.split_whitespace() {
        card.my_nums.push(num.parse::<i32>().unwrap());
    }

    count_matches(card)
}

fn count_matches(mut card: Card) -> Card {
    let mut matches = 0;
    
    for num in card.my_nums.iter() {
        if card.winning_nums.contains(num) {
            matches += 1;
        }
    }

    card.wins = matches;

    card
}

/* PART TWO */
fn part_two() {
    let content = read_file("input/input.txt");
    let mut cards: Vec<Card> = Vec::new();

    for line in content.lines() {
        cards.push(parse_line(line));
    }

    let mut index = 0;
    while index < cards.len() {
        println!("{:?}", cards[index]);
        let mut future_cards: usize = cards[index].wins as usize;
        while future_cards > 0 {
            cards[index + future_cards].copies += cards[index].copies;

            future_cards -= 1;
        }
        index += 1;
    }

    let total: u32 = cards.iter().map(|s| s.copies).sum();
    println!("Total: {}", total);

}

/* PART ONE */
fn get_card_value(card: &Card) -> i32 {
    if card.wins > 0 {
        return 2_i32.pow(card.wins-1);
        // if you like bitshift memes
        // return 1_i32 << (wins-1)
    }
    
    0
}

fn part_one() {
    let content = read_file("input/input.txt");
    let mut cards: Vec<Card> = Vec::new();
    let mut card_values: Vec<i32> = Vec::new();

    for line in content.lines() {
        cards.push(parse_line(line));
    }

    for card in cards{
        card_values.push(get_card_value(&card));
    }

    // println!("Card values: {:?}", card_values);
    
    let mut total = 0;
    for val in card_values {
        total += val;
    }
    println!("Total: {}", total);

}
