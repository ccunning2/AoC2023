use AoC2023::reader;
use std::collections::HashMap;


fn main() {
    let input = reader::read_input("input/day3.txt").unwrap();
    let part1 = day3_part1(&input);
    let part2 = day3_part2(&input);
    println!("Part 1:{}\nPart 2:{}", part1, part2);
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