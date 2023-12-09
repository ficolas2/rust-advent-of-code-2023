use std::{fs, collections::HashMap};

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn read_input(filename: &str) -> (Vec<char>, HashMap<String, (String, String)>) {
    let contents = fs::read_to_string(filename).expect("Input file does not exist");
    let mut lines = contents.lines();

    let directions = lines.next().unwrap().chars().collect();
    lines.next(); // Blank line
    
    let map = lines.map(|str|{
        let str = str
            .replace("(", "")
            .replace(")", "");
        let mut split = str.split("=");
        let location_name = split.next().unwrap().trim().to_string();

        let mut split = split.next()
            .unwrap()
            .split(",");

        let left = split.next().unwrap().trim().to_string();
        let right = split.next().unwrap().trim().to_string();

        (location_name, (left, right))
    }).collect();

    (directions, map)
}

fn part_1(directions_vec: &[char], map: &HashMap<String, (String, String)>) -> u64 {
    let mut current_location = "AAA";
    let mut steps = 0;
    'outer: loop {
        for dir in directions_vec {
            steps += 1;
            current_location = match dir {
                'L' => &map[current_location].0,
                'R' => &map[current_location].1,
                _ => panic!("Direction should be L or R.")
            };
            if current_location == "ZZZ" {
                break 'outer;
            }
        }
    }
    
    steps
}

#[derive(Debug)]
struct Ghost {
    starting_location_name: String,
    ending_location_name: String,
    cycle_length: u64,
    hit: u64
}

fn part_2(directions_vec: &[char], map: &HashMap<String, (String, String)>) -> u64 {
    let mut current_location_vec: Vec<&str> = map.iter()
        .map(|(loc, (_,_))| loc.as_str())
        .filter(|loc| loc.ends_with("A"))
        .collect();

    let mut ghost_vec: Vec<Ghost> = current_location_vec.iter()
        .map(|starting_loc| Ghost {starting_location_name: starting_loc.to_string(), ending_location_name: String::from(""), cycle_length: 0, hit: 0})
        .collect();

    for (ghost, current_location) in ghost_vec.iter_mut().zip(current_location_vec.iter_mut()) {
        let mut step = 0;
        'outer: loop{
            for dir in directions_vec {
                step += 1;
                *current_location = match dir {
                    'L' => &map[*current_location].0,
                    'R' => &map[*current_location].1,
                    _ => panic!("Direction should be L or R.")
                };

                if current_location.ends_with("Z") {
                    if ghost.hit == 0 {
                        ghost.ending_location_name = current_location.to_string();
                        ghost.hit = step;
                    } else {
                        ghost.cycle_length = step - ghost.hit;
                        break 'outer;
                    }
                }

            }

        }
    }

    for ghost in &ghost_vec {
        println!("{:?}", ghost);
    }
    // After this loop, I noticed the hit and cycle length are the same, which means, I can take
    // the LCM

    let mut iter = ghost_vec.iter();
    let starting = iter.next().unwrap().hit;
    iter.fold(starting, |running_lcm, ghost| lcm(running_lcm, ghost.hit) )

}

fn main() {
    let (directions, map) = read_input("input");

    println!("Part 1: {}", part_1(&directions, &map));
    println!("Part 2: {}", part_2(&directions, &map));

}
