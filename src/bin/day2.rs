use AoC2023::reader;
use fancy_regex::Regex;

fn main() {
    let input = reader::read_input("input/day2.txt").unwrap();
    let part1 = day2_part1(&input);
    let part2 = day2_part2(&input);
    println!("Part 1:{}\nPart 2:{}", part1, part2);
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