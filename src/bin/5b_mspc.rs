use std::{sync::{Mutex, Arc,mpsc}, thread::{scope, self, JoinHandle}, time::Instant};

use AoC2023::reader;

struct AlmanacMap {
    source: i64,
    destination: i64,
    range: i64,
}

fn main() {
    let input = reader::read_input("input/day5.txt").unwrap();
    let start_time = Instant::now();
    let part2 =  day5_part2(&input);
    println!("Part 2:{}",   part2);
    println!("Time elapsed is {:?}", start_time.elapsed());
}


fn day5_part2(input: &Vec<String>) -> i64 {
    let (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    ) = parse_input(&input);
    let new_seeds = get_part2_seeds(&seeds);
    let mut map_vec:Vec<Vec<AlmanacMap>> = Vec::new();
    map_vec.push(seed_to_soil);
    map_vec.push(soil_to_fertilizer);
    map_vec.push(fertilizer_to_water);
    map_vec.push(water_to_light);
    map_vec.push(light_to_temperature);
    map_vec.push(temperature_to_humidity);
    map_vec.push(humidity_to_location);
    let map_vec_arc = Arc::new(map_vec);
    let (tx, rx) = mpsc::channel::<i64>();
    //First will try a thread for each seed range
    let search= move |seed_range:&(i64, i64), map_vec:&Vec<Vec<AlmanacMap>>| -> i64 {
        // let map_vec = Arc::clone(&map_vec_arc);
        let (begin, end) = *seed_range;
        let mut min_destination = i64::MAX;
        for i in begin..begin+end+1 {
            let mut destination = i;
            for map in map_vec.iter() {
                destination = find_destination(&destination, &map);
            }
            //send destination back here
            if destination < min_destination {
                min_destination = destination;
            }
        }
        min_destination
    };
    for seed in new_seeds {
        let producer = tx.clone();
        let local_map = Arc::clone(&map_vec_arc);
        thread::spawn( move || {
            println!("Staring search");
            let destination = search(&seed, &*&local_map);
            producer.send(destination).unwrap();
        });
    }
    drop(tx); //The original is never actually used
   //Search messages received for min
   let mut final_min = i64::MAX;
   for received in rx {
    println!("Received a result");
        if received < final_min {
            final_min = received;
        }
   } 
    final_min
}

fn get_part2_seeds(seeds: &[i64]) -> Vec<(i64, i64)> {
    //If the index is even it's the seed number, if it's not, it's the range
    let mut range_list : Vec<(i64, i64)> = Vec::new();
    for i in 0..seeds.len() {
        if i % 2 == 0 {
            range_list.push((seeds[i], seeds[i+1]));
        }
    }
    range_list
}

fn parse_input(
    input: &[String],
) -> (
    Vec<i64>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
    Vec<AlmanacMap>,
) {
    let mut seed_to_soil: Vec<AlmanacMap> = Vec::new();
    let mut soil_to_fertilizer: Vec<AlmanacMap> = Vec::new();
    let mut fertilizer_to_water: Vec<AlmanacMap> = Vec::new();
    let mut water_to_light: Vec<AlmanacMap> = Vec::new();
    let mut light_to_temperature: Vec<AlmanacMap> = Vec::new();
    let mut temperature_to_humidity: Vec<AlmanacMap> = Vec::new();
    let mut humidity_to_location: Vec<AlmanacMap> = Vec::new();
    let mut current_map: &mut Vec<AlmanacMap> = &mut seed_to_soil;
    let mut seeds: Vec<i64> = Vec::new();
    for s in input {
        if s.contains("seeds: ") {
            let (_, numbers) = s.split_once(": ").unwrap();
            seeds = numbers
                .split(' ')
                .map(|x| x.parse::<i64>().unwrap())
                .collect();
            continue;
        } else if s.contains("seed-to-soil map") {
            current_map = &mut seed_to_soil;
            continue;
        } else if s.contains("soil-to-fertilizer map") {
            current_map = &mut soil_to_fertilizer;
            continue;
        } else if s.contains("fertilizer-to-water map") {
            current_map = &mut fertilizer_to_water;
            continue;
        } else if s.contains("water-to-light map") {
            current_map = &mut water_to_light;
            continue;
        } else if s.contains("light-to-temperature map") {
            current_map = &mut light_to_temperature;
            continue;
        } else if s.contains("temperature-to-humidity map:") {
            current_map = &mut temperature_to_humidity;
            continue;
        } else if s.contains("humidity-to-location map:") {
            current_map = &mut humidity_to_location;
            continue;
        } else if s.trim().is_empty() {
            continue;
        }
        let vals: Vec<i64> = s.split(' ').map(|x| x.parse::<i64>().unwrap()).collect();
        let almanac_map = AlmanacMap {
            source: vals[1],
            destination: vals[0],
            range: vals[2],
        };
        current_map.push(almanac_map);
    }
    (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    )
}


fn find_destination(source: &i64, map: &Vec<AlmanacMap>) -> i64 {
    for m in map {
        //Check if the source is within the range
        if *source >= m.source && *source <= (m.source + m.range) {
            //Ok, then the value will be m.destination + (source - m.source)
            return m.destination + (*source - m.source);
        }
    }
    *source
}