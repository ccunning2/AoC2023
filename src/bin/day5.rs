use std::collections::HashMap;
use AoC2023::reader;

struct AlmanacMap {
    source: i64,
    destination: i64,
    range: i64,
}

fn main() {
    let input = reader::read_input("input/debug5_1.txt").unwrap();
    let part1 = day5_part1(&input);
    println!("Part 1:{}", part1);
}

fn day5_part1(input: &Vec<String>) -> i64 {
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
    let mut map_vec:Vec<Vec<AlmanacMap>> = Vec::new();
    map_vec.push(seed_to_soil);
    map_vec.push(soil_to_fertilizer);
    map_vec.push(fertilizer_to_water);
    map_vec.push(water_to_light);
    map_vec.push(light_to_temperature);
    map_vec.push(temperature_to_humidity);
    map_vec.push(humidity_to_location);
    let mut destinations: Vec<(i64, Vec<i64>)> = Vec::new();
    for seed in seeds {
        let mut destination_list: Vec<i64> = Vec::new();
        let mut source = seed;
        for map in &map_vec {
           source= find_destination(&source, &map);
           destination_list.push(source);
        }
        destinations.push((seed, destination_list));
    }
    //now search for the min
    let mut min_val = i64::MAX;
    for d in destinations {
        let (_, list) = d;
        let destination = list.last().unwrap();
        if *destination < min_val {
            min_val = *destination;
        }
    }
    min_val
}

fn day5_part2(input: &Vec<String>) -> i64 {

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