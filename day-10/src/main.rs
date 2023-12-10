mod directions;
mod expand_map;

use std::{fs, usize};
use directions::{Direction, DIRECTIONS, get_connected_directions};
use expand_map::expand;

pub fn add((x1, y1): (usize, usize), (x2, y2): (i32, i32)) -> (usize, usize) {
    ((x1 as i32 + x2) as usize, (y1 as i32 + y2) as usize)
}

fn read_input(filename: &str) -> Vec<Vec<char>> {
    let content = fs::read_to_string(filename).expect("Input file not found");
    let content = content
        .replace("-", "─")
        .replace("L", "└")
        .replace("J", "┘")
        .replace("7", "┐")
        .replace("F", "┌");
    content.lines().map(|line| line.chars().collect()).collect()
}

fn get_starting_position(map: &Vec<Vec<char>>) -> (usize, usize) {
    map.iter()
        .enumerate()
        .filter(|(_, l)| l.contains(&'S'))
        .map(|(y, l)| (l.iter().position(|&c| c == 'S').unwrap(), y))
        .map(|(x, y)| (x, y))
        .next()
        .unwrap()
}

fn get_first_movement_positions(map: &Vec<Vec<char>>, starting_position: (usize, usize)) -> Vec<(Direction, (usize, usize))> {
    DIRECTIONS.iter()
        .map(|dir| (*dir, add(starting_position, dir.get_vector())))
        .filter(|(dir, (x, y))| {
            if *x >= map[0].len() || *y >= map.len() {
                return false;
            }
            get_connected_directions(&map[*y][*x]).contains(&dir.get_opposite())
        })
        .collect()
}

fn walk(map: &Vec<Vec<char>>, origin_dir: Direction, x: usize, y: usize) -> (Direction, (usize, usize)) {
    let connected_directions = get_connected_directions(&map[y][x]);
    let next_direction = connected_directions.iter()
        .filter(|&d| *d != origin_dir.get_opposite())
        .next()
        .unwrap();
    (*next_direction, add((x, y), next_direction.get_vector()))
}

fn part_1(map: &Vec<Vec<char>>, starting_position: (usize, usize)) -> u64 {
    let mut current_positions_vec = get_first_movement_positions(map, starting_position);
    let mut steps = 1;
    
    loop {
        steps += 1;
        let next_positions_vec: Vec<(Direction, (usize, usize))> = current_positions_vec.iter()
            .map(|(dir, (x, y))| {
                walk(map, *dir, *x, *y)
            })
        .collect();

        if next_positions_vec[0].1 == next_positions_vec[1].1 ||
            next_positions_vec[0].1 == current_positions_vec[1].1 ||
            current_positions_vec[0].1 == next_positions_vec[1].1 {
            break;
        }

        current_positions_vec = next_positions_vec;

    }

    steps
}

// Inneficient, could be solved by calcualting the area with pick's theorem
// https://en.wikipedia.org/wiki/Pick%27s_theorem
fn part_2(map: &Vec<Vec<char>>, starting_position: (usize, usize)) -> usize {
    // Create a vector of the same size as map, filled with '.'
    let mut loop_only_map: Vec<Vec<char>> = map.iter().map(|line| line.iter().map(|_| '.').collect()).collect();
    let (first_dir, first_position) = get_first_movement_positions(map, starting_position)[0];
    let (mut current_dir, mut current_position) = (first_dir, first_position);

    // Walk and store loop in an array
    loop {
        loop_only_map[current_position.1][current_position.0] = map[current_position.1][current_position.0];

        (current_dir, current_position) = walk(map, current_dir, current_position.0, current_position.1);

        if current_position == first_position {
            break;
        }
    }
    

    // expand
    let mut expanded_map = expand(&loop_only_map);
    // flood fill
    let mut todo: Vec<(usize, usize)> = Vec::new();
    todo.push((0,0));

    while todo.len() > 0 {
        let (x, y) = todo.pop().unwrap();
        if x >= expanded_map[0].len() || y >= expanded_map.len() {
            continue;
        }
        if expanded_map[y][x] == ' ' || expanded_map[y][x] == '.' {
            expanded_map[y][x] = '-';
            todo.push((x+1, y));
            todo.push((x.saturating_sub(1), y));
            todo.push((x, y+1));
            todo.push((x, y.saturating_sub(1)));
        }
    }

    // Visualization
    // for line in &mut expanded_map {
    //     for c in line {
    //         if *c == '-' {
    //             *c = ' ';
    //         }
    //     }
    // }
    // for (i, line) in expanded_map.iter().enumerate() {
    //     if i % 2 == 0 { continue; }
    //     for (j, c) in line.iter().enumerate() {
    //         if j % 2 == 0 { continue; }
    //         print!("{}", c);
    //     }
    //     print!("\n");
    // }
    
    expanded_map.iter().flatten()
        .filter(|&c| *c == '.')
        .count()
}

fn main() {
    let map = read_input("input");
    let starting_position = get_starting_position(&map);

    println!("Starting position: {:?}", starting_position);

    println!("Part 1: {}", part_1(&map, starting_position));
    println!("Part 2: {}", part_2(&map, starting_position));
}
