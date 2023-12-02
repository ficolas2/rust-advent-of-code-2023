use std::{fs, collections::HashMap};
use once_cell::sync::Lazy;

const NUMBER_MAP: Lazy<HashMap<String, char>> = Lazy::new(|| {
    [
        ("one", '1'), ("two", '2'), ("three", '3'), ("four", '4'), ("five", '5'),
        ("six", '6'), ("seven", '7'), ("eight", '8'), ("nine", '9'),
    ].iter()
        .map(|&(str, n)| (str.to_string(), n))
        .collect()
}
);

const REV_NUMBER_MAP: Lazy<HashMap<String, char>> = Lazy::new(|| {
    NUMBER_MAP.iter()
        .map(|(str, &n)| (str.chars().rev().collect::<String>(), n))
        .collect()
});



fn part_1(contents: &String) -> i32 {
    contents.lines().fold(0, |sum, line|{
        let first = line.chars()
            .find(|c| c.is_numeric())
            .unwrap();
        let last = line.chars()
            .rev()
            .find(|c| c.is_numeric())
            .unwrap();

        let number = format!("{}{}", first, last).parse::<i32>().unwrap();

        sum + number 
    } )
}

fn get_number(line: &str, number_map: &HashMap<String, char>) -> char {
    for i in 0..line.len(){
        let c = line.chars().nth(i).unwrap();
        if c.is_numeric(){
            return c;
        }
        
        for (number_str, &number_char) in number_map {
            if line[i..].starts_with(number_str) {
                return number_char;
            }
        }
    }
    '0'
}

fn part_2(contents: &String) -> i32 {
    contents.lines().fold(0, |sum, line|{
        let first = get_number(&line, &NUMBER_MAP); 
        let last = get_number(&line.chars().rev().collect::<String>(), &REV_NUMBER_MAP);
        
        let number = format!("{}{}", first, last).parse::<i32>().unwrap();

        sum + number
    })
}

fn main() {
    let contents = fs::read_to_string("input").expect("Input file not found.");

    println!("Part 1: {}", part_1(&contents));
    println!("Part 2: {}", part_2(&contents));
}
