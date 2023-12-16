use std::collections::HashMap;

use AoC2023::reader;

struct NetworkMap<'a> {
    instructions: &'a str,
    map: HashMap<String, (String, String)>,
}

fn main() {
    let input = reader::read_input("input/day8.txt").unwrap();

    let part1 = part1(&input);
    println!("Part 1 took {part1} steps");
    let part2 = part2(&input);
    println!("Part 2 took {part2} steps");
}

fn parse_input<'a>(input: &'a [String]) -> NetworkMap<'a> {
    let mut net_map = NetworkMap {
        instructions: "",
        map: HashMap::new(),
    };
    let mut add_insts = true;
    for s in input {
        if add_insts {
            net_map.instructions = s;
            add_insts = false;
            continue;
        }
        let mut key = "";
        let split1: Vec<&str> = s.split("=").collect();
        for x in split1 {
            if x.contains("(") {
                let v1 = x.replace("(", "").as_str().replace(")", "");
                let v2: Vec<&str> = v1.split(",").collect();
                let a = v2.get(0).unwrap().trim().to_owned();
                let b = v2.get(1).unwrap().trim().to_owned();
                net_map
                    .map
                    .insert(key.to_string(), (a.to_string(), b.to_string()));
            } else {
                key = x.trim();
            }
        }
    }
    net_map
}

fn part1(input: &[String]) -> i32 {
    let map = parse_input(input);
    let mut place = "AAA";
    let mut steps = 0;
    for instruction in map.instructions.chars().cycle() {
        steps += 1;
        let (left, right) = map.map.get(place).unwrap();
        if instruction == 'R' {
            place = right.as_str();
        } else {
            place = left.as_str();
        }
        if place == "ZZZ" {
            break;
        }
    }
    steps
}

fn part2(input: &[String]) -> i64 {
    let map = parse_input(input);
    //1. Get all of the keys ending with A
    let mut starters: Vec<&str> = Vec::new();
    for key in map.map.keys() {
        if key.ends_with("A") {
            starters.push(key);
        }
    }
    println!("These are the keys: {:?}", starters);

    let mut solutions: Vec<i32> = Vec::new();
    for starter in starters {
        let mut place = starter;
        let mut steps = 0;
        for instruction in map.instructions.chars().cycle() {
            steps += 1;
            let (left, right) = map.map.get(place).unwrap();
            if instruction == 'R' {
                place = right.as_str();
            } else {
                place = left.as_str();
            }
            if place.ends_with("Z"){
                solutions.push(steps);
                break;
            }
        }
    }
    //Ok, get the lcm
    let mut ans = *solutions.get(0).unwrap() as i64;
    for val in solutions.iter().skip(1) {
        ans = lcm(ans.clone() as i64, val.clone() as i64);
    }
    ans
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}
