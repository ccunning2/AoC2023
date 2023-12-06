use AoC2023::reader;
use std::collections::HashMap;

use fancy_regex::Regex;

fn main() {
    let input = reader::read_input("input/day4.txt").unwrap();
    let part1 = day4_part1(&input);
    let part2 = day4_part2(&input);
    println!("Part 1:{}\nPart 2:{}",part1, part2);
}

fn day4_part1(input: &Vec<String>) -> i32 {
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut points = 0;

    for s in input {
        let (_, info) = s.split_once(":").unwrap();
        let cards: Vec<&str> = info.split("|").collect();
        let winning_numbers: Vec<i32> = number_re.captures_iter(cards[0]).map(|x| x.unwrap().get(0).unwrap().as_str().parse::<i32>().unwrap()).collect();
        let my_numbers: Vec<i32> = number_re.captures_iter(cards[1]).map(|x| x.unwrap().get(0).unwrap().as_str().parse::<i32>().unwrap()).collect();
        let count: i32 = my_numbers.iter().filter(|&x| winning_numbers.contains(x) ).count() as i32;
        if count > 0 {
            points += 2_i32.pow((count - 1) as u32);
        }
    }
   points 
}


fn day4_part2(input: &Vec<String> ) -> i32 {
    let number_re = Regex::new(r"(\d+)").unwrap();
    let mut card = 1;
    let mut card_map: HashMap<i32, i32> = HashMap::new();
    for i in 1..input.len() + 1 {
        card_map.insert(i as i32, 1); //We start out with one of each card  
    }
    for s in input {
        let (_, info) = s.split_once(":").unwrap();
        let cards: Vec<&str> = info.split("|").collect();
        let winning_numbers: Vec<i32> = number_re.captures_iter(cards[0]).map(|x| x.unwrap().get(0).unwrap().as_str().parse::<i32>().unwrap()).collect();
        let my_numbers: Vec<i32> = number_re.captures_iter(cards[1]).map(|x| x.unwrap().get(0).unwrap().as_str().parse::<i32>().unwrap()).collect();
        let count: i32 = my_numbers.iter().filter(|&x| winning_numbers.contains(x) ).count() as i32;
        for i in card+1..card+count+1 {
            let num_of_cards = card_map.get(&card).unwrap();
            for _j in 0..*num_of_cards {
                //Add another card for i
                *card_map.entry(i).or_insert(0) += 1;
            }
        }
        card += 1;
    }
    card_map.values().sum()
}