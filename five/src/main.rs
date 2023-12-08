use std::fs;

#[derive(Debug)]
struct Range {
    dest_start: u64,
    source_start: u64,
    range: u64,
}

#[derive(Debug)]
struct Seed {
    start: u64,
    range: u64,
}

fn main() {
    part_two();
}

fn read_file(path: &str) -> String {
    fs::read_to_string(path).expect("File not found!")
}

fn parse_range(line: &str) -> Range {
    let values: Vec<&str> = line.split_whitespace().collect();

    Range {
        dest_start: values[0].parse::<u64>().unwrap(),
        source_start: values[1].parse::<u64>().unwrap(),
        range: values[2].parse::<u64>().unwrap(),
    }
}

fn parse_seeds(line: &[&str]) -> Vec<Seed> {
    let mut seeds: Vec<Seed> = Vec::new();
    let mut index = 0;
    while index < line.len() {
        seeds.push(Seed {
            start: line[index].parse::<u64>().unwrap(),
            range: line[index + 1].parse::<u64>().unwrap(),
        });
        index += 2;
    }

    seeds
}

/* PART TWO */
fn find_source(dest: u64, map: &Vec<Range>) -> u64 {
    
    for range in map {
        if dest >= range.dest_start && dest < range.dest_start + range.range {
            return range.source_start + (dest - range.dest_start);
        }
    }

    dest
}

fn contains_seed(soil: u64, seeds: &Vec<Seed>) -> bool {
    for seed in seeds {
        if soil >= seed.start && soil < seed.start + seed.range {
            return true;
        }
    }

    false
}

fn part_two() {
    let content = read_file("input/input.txt");
    let lines: Vec<&str> = content.lines().collect();

    let seeds_str: Vec<&str> = lines[0].split_whitespace().collect();
    let seeds: Vec<Seed> = parse_seeds(&seeds_str[1..]);

    let mut seed_to_soil: Vec<Range> = Vec::new();
    let mut soil_to_fert: Vec<Range> = Vec::new();
    let mut fert_to_water: Vec<Range> = Vec::new();
    let mut water_to_light: Vec<Range> = Vec::new();
    let mut light_to_temp: Vec<Range> = Vec::new();
    let mut temp_to_humid: Vec<Range> = Vec::new();
    let mut humid_to_location: Vec<Range> = Vec::new();

    let mut index = 2;
    let mut cur_build = 0;
    while index < lines.len() {
        if lines[index].contains("map") {
        } else if lines[index] != "" {
            match cur_build {
                0 => seed_to_soil.push(parse_range(&lines[index])),
                1 => soil_to_fert.push(parse_range(&lines[index])),
                2 => fert_to_water.push(parse_range(&lines[index])),
                3 => water_to_light.push(parse_range(&lines[index])),
                4 => light_to_temp.push(parse_range(&lines[index])),
                5 => temp_to_humid.push(parse_range(&lines[index])),
                6 => humid_to_location.push(parse_range(&lines[index])),
                _ => println!("Error"),
            }
        } else {
            cur_build += 1;
        }

        index += 1;
    }

    let mut cur_location = 0;
    while true {
        let location = find_source(cur_location, &humid_to_location);
        let humid = find_source(location, &temp_to_humid);
        let temp = find_source(humid, &light_to_temp);
        let light = find_source(temp, &water_to_light);
        let water = find_source(light, &fert_to_water);
        let fert = find_source(water, &soil_to_fert);
        let soil = find_source(fert, &seed_to_soil);

        if(contains_seed(soil, &seeds)){
            break;
        }

        cur_location += 1;
    }

    println!("Smallest location: {}", cur_location);

}


/* PART ONE */
fn find_dest(source: u64, map: &Vec<Range>) -> u64 {
    
    for range in map {
        if source >= range.source_start && source < range.source_start + range.range {
            return range.dest_start + (source - range.source_start);
        }
    }

    source
}

fn part_one() {
    let content = read_file("input/input.txt");
    let lines: Vec<&str> = content.lines().collect();

    let seeds_str: Vec<&str> = lines[0].split_whitespace().collect();

    let seeds: Vec<u64> = seeds_str[1..].iter().map(|x| x.parse::<u64>().unwrap()).collect();
    let mut seed_to_soil: Vec<Range> = Vec::new();
    let mut soil_to_fert: Vec<Range> = Vec::new();
    let mut fert_to_water: Vec<Range> = Vec::new();
    let mut water_to_light: Vec<Range> = Vec::new();
    let mut light_to_temp: Vec<Range> = Vec::new();
    let mut temp_to_humid: Vec<Range> = Vec::new();
    let mut humid_to_location: Vec<Range> = Vec::new();

    let mut index = 2;
    let mut cur_build = 0;
    while index < lines.len() {
        if lines[index].contains("map") {
        } else if lines[index] != "" {
            match cur_build {
                0 => seed_to_soil.push(parse_range(&lines[index])),
                1 => soil_to_fert.push(parse_range(&lines[index])),
                2 => fert_to_water.push(parse_range(&lines[index])),
                3 => water_to_light.push(parse_range(&lines[index])),
                4 => light_to_temp.push(parse_range(&lines[index])),
                5 => temp_to_humid.push(parse_range(&lines[index])),
                6 => humid_to_location.push(parse_range(&lines[index])),
                _ => println!("Error"),
            }
        } else {
            cur_build += 1;
        }

        index += 1;
    }
    
    let mut smallest_location = 9999999999999999999;
    for seed in seeds{
        let soil = find_dest(seed, &seed_to_soil);
        let fert = find_dest(soil, &soil_to_fert);
        let water = find_dest(fert, &fert_to_water);
        let light = find_dest(water, &water_to_light);
        let temp = find_dest(light, &light_to_temp);
        let humid = find_dest(temp, &temp_to_humid);
        let location = find_dest(humid, &humid_to_location);

        if location < smallest_location {
            smallest_location = location;
        }
    }

    println!("Smallest location: {}", smallest_location);
}
