use std::fs;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}
fn main() {
    part_one();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

/* PART TWO */
// Just like... remove the whitespice from the input since it's so short

/* PART ONE */
fn part_one() {
    let content = read_file("input/input.txt");
    let lines: Vec<&str> = content.lines().collect();

    let times: Vec<u64> = lines[0].split_once(":").unwrap().1.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();
    let distances: Vec<u64> = lines[1].split_once(":").unwrap().1.split_whitespace().map(|x| x.parse::<u64>().unwrap()).collect();

    let mut races: Vec<Race> = Vec::new();

    for i in 0..times.len() {
        races.push(Race {
            time: times[i],
            distance: distances[i],
        })
    }

    let mut total = 0;
    for race in races {
        if total == 0 {
            total += count_solutions(&race);
        } else {
            total *= count_solutions(&race);
        }
    }

    println!("{:?}", total);


    
}

fn count_solutions(race: &Race) -> u64 {
    let mut solutions = 0;

    for i in 0..race.time {
        let travel = (race.time - i) * i;
        if travel > race.distance {
            solutions += 1;
        }
    }

    solutions
}