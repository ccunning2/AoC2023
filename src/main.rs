mod reader;
use reader::read_input;
use regex::Regex;

fn main() {
    let input = read_input("input/day1.txt").unwrap();
    let ans1 = part1(input);
    println!("Part 1:{}", ans1);
}

fn part1(input: Vec<String>) -> u32 {
    let re = Regex::new(r"^\D*(\d).*?(\d)?\D*$").unwrap();
    let mut sum: u32 = 0;
    for s in &input {
        let captures = re.captures(s).unwrap();
        let first_num = captures.get(1).unwrap().as_str();
        let second_num = captures.get(2).unwrap_or(captures.get(1).unwrap()).as_str();
        let line_number = format!("{first_num}{second_num}").parse::<u32>().unwrap();
        sum += line_number;
    }
    sum
}
