use std::{fs, collections::HashMap, cmp::Ordering};
use once_cell::sync::Lazy;

const CARD_VALUE: Lazy<HashMap<char, u64>> = Lazy::new(|| {
    ['j', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A']
        .iter()
        .enumerate()
        .map(|(i, &c)| (c, i.try_into().unwrap()))
        .collect()
});

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five
}

struct Set {
    cards: String,
    bid: u64,
    hand_type: HandType
}

impl Set {
    fn cmp(&self, b: &Set) -> Ordering {
        match self.hand_type.cmp(&b.hand_type) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => {
                for (card_a, card_b) in self.cards.chars().into_iter().zip(b.cards.chars().into_iter()) {
                    let ordering = CARD_VALUE[&card_a].cmp(&CARD_VALUE[&card_b]);
                    if ordering == Ordering::Equal {
                        continue;
                    }
                    return ordering;
                }
                Ordering::Equal
            }
        }
    }

}

fn get_bundled_cards(cards: &str) -> Vec<(char, u64)> {
    let mut bundled_cards_map = HashMap::new();

    for card in cards.chars() {
        bundled_cards_map.entry(card).and_modify(|v| *v+=1).or_insert(1);
    }

    let mut bundled_cards = Vec::from_iter(bundled_cards_map.iter().map(|(k, v)| (*k, *v)));
    bundled_cards.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));

    bundled_cards
}

fn get_hand_type(bundled_cards: &Vec<(char, u64)>) -> HandType {
    let best_two = (bundled_cards[0].1, bundled_cards.get(1).unwrap_or(&('n', 0)).1);

    match best_two {
        (5,_) => HandType::Five,
        (4,_) => HandType::Four,
        (3,2) => HandType::FullHouse,
        (3,_) => HandType::Three,
        (2,2) => HandType::TwoPair,
        (2,_) => HandType::Pair,
        (_,_) => HandType::HighCard
    }
}

fn read_part_1(filename: &str) -> Vec<Set> {
    let contents = fs::read_to_string(filename).expect("Input file not found.");
    let line_iter = contents.lines();

    line_iter.map(|line|{
        let mut split = line.split(" ");
        let cards = split.next().unwrap().to_string();
        let bid = split.next().unwrap().parse().unwrap();
        let bundled_cards = get_bundled_cards(&cards);
        let hand_type = get_hand_type(&bundled_cards);

        Set { cards, bid, hand_type }
    }).collect()
}

fn read_part_2(filename: &str) -> Vec<Set> {
    let contents = fs::read_to_string(filename).expect("Input file not found.");
    let line_iter = contents.lines();

    line_iter.map(|line|{
        let mut split = line.split(" ");
        let cards = split.next().unwrap().to_string().replace("J", "j");
        let bid = split.next().unwrap().parse().unwrap();

        let joker_count = cards.chars().filter(|c| *c=='j').count();
        let mut bundled_cards = get_bundled_cards(&cards);

        let best_bundle = bundled_cards.iter_mut()
            .filter(|(c, _)| *c != 'j')
            .next();
        if let Some((_, count)) = best_bundle {
            *count += joker_count as u64;

            let j_bundle = bundled_cards.iter_mut()
                .filter(|(c, _)| *c == 'j')
                .next();
            if let Some((_, j_count)) = j_bundle {
                *j_count = 0;
            }
        }
        bundled_cards.sort_by(|(_, v1), (_, v2)| v2.cmp(v1));



        let hand_type = get_hand_type(&bundled_cards);

        Set { cards, bid, hand_type }
    }).collect()
}

fn calculate(set_vec: &Vec<Set>) -> u64 {
    set_vec.iter().enumerate().fold(0, |sum, (i, set)| {
        sum + set.bid * ((i + 1) as u64)
    })
}

fn main() {
    let mut part_1_vec = read_part_1("input");
    part_1_vec.sort_by(|a, b| a.cmp(b));
    println!("Part 1: {}", calculate(&part_1_vec));

    let mut part_2_vec = read_part_2("input");
    part_2_vec.sort_unstable_by(|a, b| a.cmp(b));
    println!("Part 2: {}", calculate(&part_2_vec));
}
