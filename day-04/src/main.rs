use std::{fs, collections::HashSet};

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
}

fn read_input(filename: &str) -> Vec<Card> {
    let contents = fs::read_to_string(filename).expect("Input file not found.");
    let line_vec: Vec<&str> = contents.lines().collect();

    line_vec.iter().map(|line| {
        // Throw away the card number, by splitting by :
        let mut split = line.split(":").nth(1).unwrap().split("|");

        let winning_str = split.next().unwrap().trim();
        let number_str = split.next().unwrap().trim();

        let winning_numbers = winning_str
            .split(" ")
            .filter(|str| !str.is_empty())
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        let numbers = number_str 
            .split(" ")
            .filter(|str| !str.is_empty())
            .map(|str| str.parse::<u32>().unwrap())
            .collect();
        
        Card {
            winning_numbers,
            numbers
        }
    }).collect()
}

fn get_winning(card: &Card) -> usize {
    card.numbers.intersection(&card.winning_numbers).count()
}

fn part_1(card_vec: &Vec<Card>) -> u32 {
    
    card_vec.iter().map(|card| {
        let winning_count = get_winning(card);


        if winning_count > 0 {
            2_u32.pow((winning_count - 1).try_into().unwrap())
        }
        else {
            0
        }
    }).sum()

}

fn part_2(card_vec: &mut Vec<Card>) -> u64 {
    let mut count_vec = vec![1; card_vec.len()];
    for (i, card) in card_vec.iter().enumerate() {
        let winning_count = get_winning(card);

        for won_index in i+1..i+1+winning_count {
            if won_index >= count_vec.len() {
                break;
            }
            count_vec[won_index] += count_vec[i];
        }

    }

    count_vec.iter().map(|n| *n as u64).sum()
}

fn main() {
    let mut card_vec = read_input("input");

    println!("Part 1: {}", part_1(&card_vec));
    println!("Part 2: {}", part_2(&mut card_vec))


}
