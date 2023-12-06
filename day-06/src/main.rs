use std::fs;


struct Race {
    time: u64,
    distance: u64
}

fn solve_quadratic(a: f64, b:f64, c: f64) -> (f64, f64){
    let discriminant = b * b - 4.0 * a * c;

    if discriminant >= 0.0 {
        let sqrt = discriminant.sqrt();
        ( 
            (-b + sqrt) / (2.0 * a),
            (-b - sqrt) / (2.0 * a)
        )
    } else {
        (0.0, 0.0)
    }
    
}

fn read_input(filename: &str) -> Vec<Race> {
    let contents = fs::read_to_string(filename).expect("Input file not found.");
    let mut line_vec = contents.lines();

    let line1 = line_vec.next().unwrap();
    let line2 = line_vec.next().unwrap();

    let mut  iter = line1.split_whitespace().zip(line2.split_whitespace());
    iter.next(); // Throw away first line 
    
    iter.map(|(time, distance)| Race {
            time: time.parse().unwrap(),
            distance: distance.parse().unwrap()
        }).collect()
}

fn get_ways(race: &Race) -> u64 {
    // Calculate the limit times, d=(t-x)*x
    // 0=-x^2+tx-d; a=-1; b=t; c=-d
    let (sol_1, sol_2) = solve_quadratic(-1.0, race.time as f64, -(race.distance as f64));

    let lower_bound_f = f64::min(sol_1, sol_2);
    let lower_bound = lower_bound_f.ceil() as u64;
    let upper_bound_f = f64::max(sol_1, sol_2);
    let upper_bound = upper_bound_f.ceil() as u64;

    let mut count = upper_bound - lower_bound;
    if lower_bound_f == lower_bound_f - lower_bound_f % 1.0 {
        count -= 1;
    }
    count
}

fn part_1(race_vec: &Vec<Race>) -> u64 {
    race_vec.iter().fold(1, |sum, race| {
        sum * get_ways(race) 
    })
}

fn part_2(race_vec: &Vec<Race>) -> u64 {
    let joined_time = race_vec.iter()
        .fold(String::from(""), |str, race| format!("{}{}", str, race.time.to_string()))
        .parse()
        .unwrap();
    let joined_distance = race_vec.iter()
        .fold(String::from(""), |str, race| format!("{}{}", str, race.distance.to_string()))
        .parse()
        .unwrap();

    let joined_race = Race{
        time: joined_time,
        distance: joined_distance
    };

    part_1(&vec![joined_race])

}

fn main() {
    let race_vec = read_input("input");
    
    println!("Part 1: {}", part_1(&race_vec));
    println!("Part 2: {}", part_2(&race_vec));

}
