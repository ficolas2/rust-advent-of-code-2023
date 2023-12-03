use regex::Regex;
use std::fs;
use once_cell::sync::Lazy;

const NUMBER_PATTERN: Lazy<Regex> = Lazy::new(|| {Regex::new(r"\d+").unwrap()});

fn part_1(line_vec: &Vec<&str>) -> i32 {
    let mut sum = 0;
    for (num_y, line) in line_vec.iter().enumerate() {
        for mat in NUMBER_PATTERN.find_iter(line) {
            let start = mat.start();
            let end = mat.end();

            let mut match_found = false;
            'outer: for y in num_y.saturating_sub(1)..=(num_y + 1) {
                let iterating_line = match line_vec.get(y) {
                    None => continue,
                    Some(i) => i,
                };

                for x in start.saturating_sub(1)..(end + 1) {
                    let char = iterating_line.chars().nth(x);
                    match char {
                        None => continue,
                        Some(i) => {
                            if i.is_numeric() || i == '.' {
                                continue;
                            }
                            match_found = true;
                            break 'outer;
                        }
                    }
                }
            }
            if match_found {
                sum += mat.as_str().parse::<i32>().unwrap();
            }
        }
    }

    sum
}

// Slow af, but simple solution
fn part_2(line_vec: &Vec<&str>) -> i32 {
    let mut sum = 0;
    for (cog_y, line) in line_vec.iter().enumerate() {
        for (cog_x, _) in line.chars().enumerate().filter(|&(_, c)| c == '*') {
            //Iterate over lines around the cog
            let mut close_numbers: Vec<i32> = Vec::new();
            for y in cog_y.saturating_sub(1)..=(cog_y + 1) {
                let iterating_line = match line_vec.get(y) {
                    None => continue,
                    Some(i) => i,
                };

                for mat in NUMBER_PATTERN.find_iter(iterating_line) {
                    if cog_x >= mat.start().saturating_sub(1) && cog_x <= mat.end() {
                        close_numbers.push(mat.as_str().parse::<i32>().unwrap());
                    }
                }

            }
            if close_numbers.len() == 2 {
                sum += close_numbers[0] * close_numbers[1];
            }
        }
    }

    sum
}

fn main() {
    let contents = fs::read_to_string("input").expect("Input file not found.");
    let lines: Vec<&str> = contents.lines().collect();

    println!("Part 1: {}", part_1(&lines));
    println!("Part 2: {}", part_2(&lines));
}
