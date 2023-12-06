use std::fs;

#[derive(Debug)]
struct Hand {
    red_cubes: i32,
    blue_cubes: i32,
    green_cubes: i32,
}

impl Hand {
    fn power(&self) -> i32 {
        self.blue_cubes * self.red_cubes * self.green_cubes
    }
}

#[derive(Debug)]
struct Game {
    game_number: i32, 
    hands: Vec<Hand>,
}

fn main() {
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn parse_hand(hand_str: &str) -> Hand {
    let mut hand: Hand = Hand { red_cubes: 0, blue_cubes: 0, green_cubes: 0, };

    for cube in hand_str.split(',') {
        if cube.contains("green") {
            hand.green_cubes = cube.split_whitespace().next().unwrap().parse::<i32>().unwrap();
        } else if cube.contains("red") {
            hand.red_cubes = cube.split_whitespace().next().unwrap().parse::<i32>().unwrap();
        } else if cube.contains("blue") {
            hand.blue_cubes = cube.split_whitespace().next().unwrap().parse::<i32>().unwrap();
        }
    }

    hand
}

fn parse_game(game_str: &str) -> Game {
    let colon = game_str.find(":").unwrap();

    let game_number: i32 = (&game_str[..colon]).split_whitespace().last().unwrap().parse::<i32>().unwrap();
    let mut hands: Vec<Hand> = Vec::new();

    let hand_string = &game_str[colon+1..];
    for hand in hand_string.split(";") {
        hands.push(parse_hand(hand));
    }

    Game {
        game_number: game_number,
        hands: hands,
    }

}

fn get_mini_power(game: &Game) -> i32 {
    let mut min_hand: Hand = Hand { red_cubes: 0, blue_cubes: 0, green_cubes: 0, };

    for hand in &game.hands {
        if hand.red_cubes > min_hand.red_cubes {
            min_hand.red_cubes = hand.red_cubes;
        }

        if hand.blue_cubes > min_hand.blue_cubes {
            min_hand.blue_cubes = hand.blue_cubes;
        }

        if hand.green_cubes > min_hand.green_cubes {
            min_hand.green_cubes = hand.green_cubes;
        }
    }

    min_hand.power()
}

fn part_two() {
    let file_path = "input/input.txt";
    let content = read_file(file_path);

    let mut games: Vec<Game> = Vec::new();

    for line in content.lines() {
        games.push(parse_game(line));
    }

    let mut total = 0;
    for game in games {
        total += get_mini_power(&game);
    }
    println!("total {}", total);
}

fn is_possible(game: &Game) -> bool {
    let reds = 12;
    let greens = 13;
    let blues = 14;

    let mut overall_possible = true;
    for hand in &game.hands {
        if hand.red_cubes > reds || hand.blue_cubes > blues || hand.green_cubes > greens {
            overall_possible = false;
        }
    }

    overall_possible
}

fn part_one() {
    let file_path = "input/input.txt";
    let content = read_file(file_path);

    let mut games: Vec<Game> = Vec::new();

    for line in content.lines() {
        games.push(parse_game(line));
    }

    let mut total = 0;
    for game in games {
        if is_possible(&game) {
            total += game.game_number;
        }
    }
    println!("total {}", total);
}