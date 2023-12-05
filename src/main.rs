mod reader;

use reader::read_input;
use fancy_regex::Regex;
use std::collections::HashMap;

fn main() {
    let input = read_input("input/day3.txt").unwrap();
    // let ans = day2_part1(&input);
    // let ans2 = day2_part2(&input);
    // let _x = get_symbol_indices(&input);

    // let ans1 = day3_part1(&input);
    // println!("{}", ans1);

    let ans2 = day3_part2(&input);
    println!("{}", ans2);
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

fn get_symbol_indices_day3(input: &Vec<String>) -> (HashMap<i32, Vec<i32>> , Vec<(i32, i32, i32,i32)>) {
    //Store symbols in this
    let mut symbol_map: HashMap<i32, Vec<i32>> = HashMap::new();
    //Store number data in the form (number, row, begin_index, end_index)
    let mut number_data: Vec<(i32, i32, i32,i32)> = Vec::new();
    let mut row = 0;
    let mut number = String::from("");
    let mut begin_index = 0;
    let mut end_index = 0;
    for s in input {
        for (i, c) in s.char_indices() {
            match c as i32 {
                _ if (c as i32) < 45 || (c as i32) > 58 || c == '/' || c == '-' => {
                    let ints = symbol_map.entry(row).or_insert(Vec::new());
                    ints.push(i as i32);
                    if number.len() > 0 {
                        end_index = if i > 0 {i - 1 } else {s.len() - 1};
                        number_data.push((
                            number.parse::<i32>().unwrap(),
                            if i == 0 {row - 1} else {row},
                            begin_index,
                            end_index as i32
                        ));
                        number.clear();
                        begin_index = 0;
                        end_index = 0;
                    }
                }
                _ if (c as i32) > 47 => { //Otherwise we have a number
                    if number.len() == 0 {
                        begin_index = i as i32;
                    }
                    number.push(c);
                } 
                _ => {
                    if number.len() > 0 {
                        end_index = if i > 0 {i - 1 } else {s.len() - 1};
                        number_data.push((
                            number.parse::<i32>().unwrap(),
                            if i == 0 {row - 1} else {row},
                            begin_index,
                            end_index as i32
                        ));
                        number.clear();
                        begin_index = 0;
                        end_index = 0;
                    }
                }
            }
        }
        row += 1;
    }
    (symbol_map , number_data)
}

fn get_symbol_indices_day3_2(input: &Vec<String>) -> (HashMap<i32, Vec<i32>> , Vec<(i32, i32, i32,i32)>) {
    //Store symbols in this
    let mut symbol_map: HashMap<i32, Vec<i32>> = HashMap::new();
    //Store number data in the form (number, row, begin_index, end_index)
    let mut number_data: Vec<(i32, i32, i32,i32)> = Vec::new();
    let mut row = 0;
    let mut number = String::from("");
    let mut begin_index = 0;
    let mut end_index = 0;
    for s in input {
        for (i, c) in s.char_indices() {
            match c as i32 {
                _ if c == '*' => { //only save location of stars
                    let ints = symbol_map.entry(row).or_insert(Vec::new());
                    ints.push(i as i32);
                    if number.len() > 0 {
                        end_index = if i > 0 {i - 1 } else {s.len() - 1};
                        number_data.push((
                            number.parse::<i32>().unwrap(),
                            if i == 0 {row - 1} else {row},
                            begin_index,
                            end_index as i32
                        ));
                        number.clear();
                        begin_index = 0;
                        end_index = 0;
                    }
                }
                _ if (c as i32) > 47  && (c as i32) < 58 => { //Otherwise we have a number
                    if number.len() == 0 {
                        begin_index = i as i32;
                    }
                    number.push(c);
                } 
                _ => {
                    if number.len() > 0 {
                        end_index = if i > 0 {i - 1 } else {s.len() - 1};
                        number_data.push((
                            number.parse::<i32>().unwrap(),
                            if i == 0 {row - 1} else {row},
                            begin_index,
                            end_index as i32
                        ));
                        number.clear();
                        begin_index = 0;
                        end_index = 0;
                    }
                }
            }
        }
        row += 1;
    }
    (symbol_map , number_data)
}
fn day3_part1(input: &Vec<String>) -> i32 {
    let (symbol_map, number_data) = get_symbol_indices_day3(input);
    let empty_vec: Vec<i32> = Vec::new();
    let mut sum = 0;
    for num_data in number_data {
        let (number, row, begin_index, end_index) = num_data;
        //Check above
        if row > 0 {
            if symbol_map.get(&(row-1)).unwrap_or(&empty_vec).iter().filter(|x| x >= &&(begin_index - 1)  && x<= &&(end_index + 1) ).count() > 0 {
                sum += number;
                continue;
            }
        }
        //Check below
        if row < ( input.len() as i32  - 1) {
            if symbol_map.get(&(row+1) ).unwrap_or(&empty_vec).iter().filter(|x| x >= &&(begin_index - 1)  && x<= &&(end_index + 1) ).count() > 0 {
                sum += number;
                continue;
            }
        }
        //Check right
        if symbol_map.get(&row ).unwrap_or(&empty_vec).iter().any(|x| x == &(end_index + 1) ) {
            sum += number;
            continue;
        }
        //Check Left
        if symbol_map.get(&row ).unwrap_or(&empty_vec).iter().any(|x| x == &(begin_index - 1) ) {
            sum += number;
            continue;
        }
    } 
    sum
}

fn day3_part2(input: &Vec<String>) -> i32 {


    let (symbol_map, number_data) = get_symbol_indices_day3_2(input);

    let mut sum = 0;

    for (row,value) in symbol_map {
        for col in value {
            //For each gear location, look above, down,left,right -- there must be exactly two numbers

            let mut number_list: Vec<i32> = Vec::new();
            //Look above
            if row > 0 {
               let above: Vec<i32> =  number_data.iter().filter(|&&x|  {
                    let (number, num_row, begin, end) = x;
                    num_row == row -1
                    &&
                    col >= (begin -1) && col <= (end + 1)
               } )
               .map(|&x| {
                let (number, _, _, _) = x;
                number
               } )
               .collect();
               number_list.extend(above);
            }
            //Look below
            if row < ( input.len()  - 1).try_into().unwrap() {
            let below: Vec<i32> = number_data.iter().filter(|&&x|  {
                    let (number, num_row, begin, end) = x;
                    num_row == row +1
                    &&
                    col >= (begin -1) && col <= (end + 1)
               } )
               .map(|&x| {
                let (number, _, _, _) = x;
                number
               } )
               .collect();
               number_list.extend(below);
            }
            //Look right and left
            let sides: Vec<i32> = number_data.iter().filter(|&&x| {
                let (number, num_row, begin, end) = x;
                num_row == row
                &&
                (col == (begin -1) || col == (end + 1))
            })
           .map(|&x| {
            let (number, _,_,_) = x;
            number
           }) 
            .collect();
            number_list.extend(sides);

            //If there's exactly 2, multiply together and add to sum
            if number_list.len() == 2 {
                sum += number_list[0] * number_list[1];
            }
        }
    }
    println!("Debug");
sum
}