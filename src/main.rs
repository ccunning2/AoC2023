mod reader;

use reader::read_input;
use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = read_input("input/day2.txt").unwrap();
    let ans = day2_part1(&input);
    let ans2 = day2_part2(&input);
    println!("{}, {}", ans, ans2);
}

fn day1_part1(input: &Vec<String>) -> u32 {
    let re = Regex::new(r"^\D*(\d).*?(\d)?\D*$").unwrap();
    let mut sum: u32 = 0;
    for s in input {
        let captures = re.captures(s).unwrap().unwrap();
        let first_num = captures.get(1).unwrap().as_str();
        let second_num = captures.get(2).unwrap_or(captures.get(1).unwrap()).as_str();
        let line_number = format!("{first_num}{second_num}").parse::<u32>().unwrap();
        sum += line_number;
    }
    sum
}

fn find_matches(line: String) -> (String, String) {
    let re = Regex::new(r"(?=(\d|zero|one|two|three|four|five|six|seven|eight|nine))").unwrap();
    //Want to find all matches, get the first and the last
    let mut matches: Vec<&str> = Vec::new();
    let allcaptures = re.captures_iter(&line);
    for cap in allcaptures {
        let cap_matches = cap.unwrap();
        let cap_str = cap_matches.get(1).unwrap().as_str();
        matches.push(cap_str);
    }
    let first = matches[0];
    let last = matches[matches.len() - 1];
    (String::from(first), String::from(last))
}

fn day1_part2(input: &Vec<String>) -> u32 {
    let mut sum: u32 = 0;
    //Init a conversion map
    let mut numbers: HashMap<String, String> = HashMap::new();

    numbers.insert("zero".to_string(), "0".to_string());
    numbers.insert("one".to_string(), "1".to_string());
    numbers.insert("two".to_string(), "2".to_string());
    numbers.insert("three".to_string(), "3".to_string());
    numbers.insert("four".to_string(), "4".to_string());
    numbers.insert("five".to_string(), "5".to_string());
    numbers.insert("six".to_string(), "6".to_string());
    numbers.insert("seven".to_string(), "7".to_string());
    numbers.insert("eight".to_string(), "8".to_string());
    numbers.insert("nine".to_string(), "9".to_string());
    for s in input {
        let (first, second) = find_matches(s.clone());
        let converted_first = numbers.get(first.as_str()).unwrap_or(&first);
        let converted_second = numbers.get(second.as_str()).unwrap_or(&second);
        let actual = format!("{}{}", converted_first, converted_second)
            .parse::<u32>()
            .unwrap();
        sum += actual;
    }
    sum
}

fn day2_part1(input: &Vec<String>) -> u32 {
    let mut ans: u32 = 0;
    for s in input {
        //1. Parse game id and game data
        let (id, data) = get_game_data1(s);
        if !invalid_game1(data) {
            ans += id.parse::<u32>().unwrap();
        }
    }
    ans
}

fn day2_part2(input: &Vec<String>) -> u32 {
    let mut ans: u32 = 0;
    for s in input {
        //1. Parse game id and game data
        let (_, data) = get_game_data1(s);
        let power = get_game_power(data);
        ans += power;
    }
    ans
}

fn get_game_data1(s: &String) -> (&str, Vec<&str>) {
    let re = Regex::new(r"Game (?<id>\d+):(?<data>.*)").unwrap();
    let caps =re.captures(s).unwrap().unwrap();
    let id = caps.name("id").unwrap().as_str();
    let data : Vec<&str> = caps.name("data").unwrap().as_str().split(";").collect();
    (id, data)
}

fn invalid_game1(data: Vec<&str>) -> bool {
    let max_red = 12;
    let max_green = 13;
    let max_blue = 14;
    for s in data {
        let games = s.split(","); 
        for game in games {
            let mut split = game.split_whitespace();
            let number: u32 = split.next().unwrap().parse().unwrap();
            let color: &str = split.next().unwrap();
            match (number, color) {
                (n , c ) if c == "red" && n > max_red => return true,
                (n , c ) if c == "green" && n > max_green => return true,
                (n , c ) if c == "blue" && n > max_blue => return true,
                _ => {}
            }
        }
    }
    false
}

fn get_game_power(data: Vec<&str>) -> u32 {
    let mut min_red = 0;
    let mut min_green = 0;
    let mut min_blue = 0;
    for s in data {
        let games = s.split(","); 
        for game in games {
            let mut split = game.split_whitespace();
            let number: u32 = split.next().unwrap().parse().unwrap();
            let color: &str = split.next().unwrap();
            match (number, color) {
                (n , c ) if c == "red" && n > min_red => min_red = n,
                (n , c ) if c == "green" && n > min_green => min_green = n,
                (n , c ) if c == "blue" && n > min_blue => min_blue = n,
                _ => {}
            }
        }
    }
    min_blue*min_green*min_red
}