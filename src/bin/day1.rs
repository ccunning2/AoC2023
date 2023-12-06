use AoC2023::reader;
use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = reader::read_input("input/day1.txt").unwrap();
    let part1 = day1_part1(&input);
    let part2 = day1_part2(&input);
    println!("Part 1:{}\nPart 2:{}", part1, part2);
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