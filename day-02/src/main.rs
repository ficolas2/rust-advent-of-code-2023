use std::{fs, collections::HashMap};


struct Game {
    id: i32,
    draws: Vec<HashMap<String, i32>>
}

fn read_games() -> Vec<Game> {
    let contents = fs::read_to_string("input").expect("Input file not found.");
    let mut game_vec: Vec<Game> = Vec::new();


    for (i, line) in contents.lines().enumerate() {
        let id: i32 = (i+1).try_into().unwrap();

        let draws: Vec<HashMap<String, i32>> = line[line.find(':').unwrap() + 1..]
            .split(";")
            .map(|str| str.split(",").map(str::trim))
            .map(|draws| {
                draws.map(|str| {
                    let mut cube = str.split(" ");
                    let amount = cube.next().unwrap().parse::<i32>().unwrap();
                    let color = cube.next().unwrap().to_string();

                    (color, amount)
                }).collect()
            })
            .collect();

        game_vec.push(Game { id, draws });
    }

    game_vec
}

fn part_1(game_vec: &Vec<Game>, max_cubes: &HashMap<&str, i32>) -> i32 {
    game_vec.iter().fold(0, |sum, game| {
        let valid = game.draws.iter().all(|draw| {
            draw.iter().all(|(color, &amount)| max_cubes[color as &str] >= amount)
        });

        if valid {
            sum + game.id
        }
        else {
            sum
        }
    })
}

fn part_2(game_vec: &Vec<Game>) -> i32{
    
    game_vec.iter().fold(0, |sum, game| {
        let mut maxes: HashMap<&str, i32> = HashMap::new();

        for draw in &game.draws {
            for (color, count) in draw {
                if maxes.get(color as &str).unwrap_or(&0) < count {
                    maxes.insert(color, *count);
                }
            }
        }
        
        let power = maxes.iter().fold(1, |mul, (_, count)| {
            count * mul
        });

        sum + power
    })
    
}

fn main() {
    let max_cubes: HashMap<&str, i32>  = [("red", 12), ("green", 13), ("blue", 14)]
        .iter().cloned().collect();
    let game_vec = read_games();
    
    println!("Part 1: {}", part_1(&game_vec, &max_cubes));
    println!("Part 2: {}", part_2(&game_vec));
}

