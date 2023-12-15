use std::{collections::HashMap, cmp::Ordering};

use AoC2023::reader;

#[derive(Debug)]
enum HandType {
    HighCard,
    Pair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind
}

impl HandType {
    fn rank(&self) -> i32 {
        match self {
            HandType::HighCard => 1,
            HandType::Pair => 2,
            HandType::TwoPair => 3,
            HandType::ThreeOfAKind => 4,
            HandType::FullHouse => 5,
            HandType::FourOfAKind => 6,
            HandType::FiveOfAKind => 7
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: String,
    bid: i32,
    hand_type: HandType
}

fn main() {
    let input = reader::read_input("input/day7.txt").unwrap();

    let part1: i32 = part1(&input);
    let part2 = part2(&input);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn convert_card(c: char) -> i32 {
    if c.is_alphabetic() {
        match c {
            'T' => 10,
            'J' => 11,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
             _  => panic!("Weird Card") 
        }
    } else {
        c.to_string().parse::<i32>().unwrap()
    }
}

fn convert_card2(c: char) -> i32 {
    if c.is_alphabetic() {
        match c {
            'T' => 10,
            'J' => 1,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
             _  => panic!("Weird Card") 
        }
    } else {
        c.to_string().parse::<i32>().unwrap()
    }
}

fn part1(input: &[String]) -> i32 {
    
    let mut parsed_cards: Vec<Hand> = Vec::new();
    //Go through cards and determine strength
    for line in input {
        let mut hand_parts = line.split_whitespace();
        let (cards, bid) = (hand_parts.next().unwrap(), hand_parts.next().unwrap().parse::<i32>().unwrap() );
        let mut card_map: HashMap<char, i32> = HashMap::new(); 
        for card in cards.chars() {
            *card_map.entry(card).or_insert(0) += 1;
        }
        //Determine what hand we have
        let mut hand_type: HandType = HandType::HighCard;
        if card_map.values().any(|x| *x == 5) {
            hand_type = HandType::FiveOfAKind;
        } else if card_map.values().any(|x| *x == 4) {
            hand_type = HandType::FourOfAKind;
        } else if card_map.values().filter(|x| **x == 3 || **x == 2).sum::<i32>() == 5 {
            hand_type = HandType::FullHouse;
        } else if card_map.values().any(|x| *x == 3) {
            hand_type = HandType::ThreeOfAKind;
        } else if card_map.values().filter(|x| **x == 2).count() == 2 {
            hand_type = HandType::TwoPair;
        } else if card_map.values().any(|x| *x == 2) {
            hand_type = HandType::Pair;
        } else {
            hand_type = HandType::HighCard;
        }
        parsed_cards.push(Hand { cards: cards.to_string(), bid: bid.clone(), hand_type })
    }
    parsed_cards.sort_by(|a , b| {
        match a.hand_type.rank().cmp(&b.hand_type.rank()) {
            Ordering::Equal => {
                let mut counter = 0;
                let a_cards: Vec<char> = a.cards.chars().collect();
                let b_cards: Vec<char> = b.cards.chars().collect();
                while a_cards.get(counter) == b_cards.get(counter) {
                    counter += 1;
                }
                convert_card(*(a_cards.get(counter).unwrap())).cmp(&(convert_card(*(b_cards.get(counter).unwrap()))))
            },
            other => other
        }
    });
    let mut sum = 0;
    for x in 1..parsed_cards.len()+1 {
        let bid = parsed_cards.get(x-1).unwrap().bid;
        sum += bid * x as i32;
    }
    sum
}


fn part2(input: &[String]) -> i32 {
    
    let mut parsed_cards: Vec<Hand> = Vec::new();
    //Go through cards and determine strength
    for line in input {
        let mut hand_parts = line.split_whitespace();
        let (cards, bid) = (hand_parts.next().unwrap(), hand_parts.next().unwrap().parse::<i32>().unwrap() );
        let mut card_map: HashMap<char, i32> = HashMap::new(); 
        let mut j_count = 0;
        for card in cards.chars() {
            if card != 'J' {
            *card_map.entry(card).or_insert(0) += 1;
            } else {
                j_count += 1;
            }
        }

        //Determine what hand we have
        let mut hand_type: HandType = HandType::HighCard;
        let any_pair = card_map.values().any(|x| *x == 2);
        let two_pair = card_map.values().filter(|x| **x == 2).count() == 2;
        let three_of_a_kind = card_map.values().any(|x| *x == 3);
        let four_of_a_kind = card_map.values().any(|x| *x == 4);
        let five_of_a_kind = card_map.values().any(|x| *x == 5);
        if five_of_a_kind || (four_of_a_kind && (j_count == 1)) || (three_of_a_kind && (j_count == 2)) || (any_pair && (j_count == 3)) || (j_count == 4) || (j_count == 5) {
            hand_type = HandType::FiveOfAKind;
        } else if four_of_a_kind || (three_of_a_kind && (j_count == 1)) || (any_pair && (j_count == 2)) || (j_count == 3) {
            hand_type = HandType::FourOfAKind;
        } else if (any_pair && three_of_a_kind) || (two_pair && (j_count == 1)){
            hand_type = HandType::FullHouse;
        }  else if three_of_a_kind || (any_pair && (j_count == 1)) || (j_count == 2) {
            hand_type = HandType::ThreeOfAKind;
        } else if two_pair {
            hand_type = HandType::TwoPair;
        } else if any_pair || (j_count == 1) {
            hand_type = HandType::Pair;
        } else {
            hand_type = HandType::HighCard;
        }
        parsed_cards.push(Hand { cards: cards.to_string(), bid: bid.clone(), hand_type })
    }
    parsed_cards.sort_by(|a , b| {
        match a.hand_type.rank().cmp(&b.hand_type.rank()) {
            Ordering::Equal => {
                let mut counter = 0;
                let a_cards: Vec<char> = a.cards.chars().collect();
                let b_cards: Vec<char> = b.cards.chars().collect();
                while a_cards.get(counter) == b_cards.get(counter) {
                    counter += 1;
                }
                convert_card2(*(a_cards.get(counter).unwrap())).cmp(&(convert_card2(*(b_cards.get(counter).unwrap()))))
            },
            other => other
        }
    });
    let mut sum = 0;
    for x in 1..parsed_cards.len()+1 {
        let bid = parsed_cards.get(x-1).unwrap().bid;
        sum += bid * x as i32;
    }
    sum
}