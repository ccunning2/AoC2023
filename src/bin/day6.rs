use AoC2023::reader;

fn main() {
    let input = reader::read_input("input/day6.txt").unwrap();

    let total = part1(&input);
    println!("Total Part 1:{}", total);
	 let total2 = part2(&input);
	 println!("Total Part 2:{}", total2);

}

fn part2(input: &Vec<String>) -> i64 {
    let races = parse_races2(&input);
    let total = calculate_total_2(&races);
    total
}

fn part1(input: &Vec<String>) -> i32 {
    let races = parse_races1(&input);

    let total = calculate_total_1(&races);
    total
}

fn calculate_total_1(races: &Vec<(i32, i32)>) -> i32 {
    let mut total = 1;
    for r in races {
        let (race_time, distance_to_beat) = r;
        //Each second spent holding down the button increases speed by 1 m/s
        let mut ways_to_win = 0;
        for time_holding_button in 0..*race_time {
            let time_remaining = race_time - time_holding_button;
            let distance = time_holding_button * time_remaining;
            if distance > *distance_to_beat {
                ways_to_win += 1;
            }
        }
        total *= ways_to_win;
    }
    total
}

fn parse_races1(input: &Vec<String>) -> Vec<(i32, i32)> {
    let mut races: Vec<(i32, i32)> = Vec::new();

    for s in input {
        //First row is time, second is Distance
        let cols: Vec<&str> = s
            .split(" ")
            .filter(|x| !x.is_empty() && matches!(x.parse::<i32>(), Ok(_)))
            .collect();
        for i in 0..cols.len() {
            let value = cols[i].parse::<i32>().unwrap();
            if let Some(_) = races.get(i) {
                races[i].1 = value;
            } else {
                races.push((value, 0));
            }
        }
    }
    races
}


fn parse_races2 (input: &Vec<String>) -> (i64, i64) {
	
	let mut races:(i64,i64) = (0,0);
    for s in input {
        //First row is time, second is Distance
        let cols: Vec<&str> = s
            .split(" ")
            .filter(|x| !x.is_empty() && matches!(x.parse::<i32>(), Ok(_)))
            .collect();
		let the_value = cols.join("").parse::<i64>().unwrap();
		if races.0 == 0 {
			races.0 = the_value;
		} else {
			races.1 = the_value;
		}
    }
	races
}

fn calculate_total_2(races: &(i64, i64)) -> i64 {

    let mut total = 0;
        let (race_time, distance_to_beat) = races;
        //Each second spent holding down the button increases speed by 1 m/s
        for time_holding_button in 0..*race_time {
            let time_remaining = race_time - time_holding_button;
            let distance = time_holding_button * time_remaining;
            if distance > *distance_to_beat {
                total += 1;
            }
        }
    
    total
}