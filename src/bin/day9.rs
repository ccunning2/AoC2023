use AoC2023::reader;


fn main() {
    let input = reader::read_input("input/day9.txt").unwrap();
    let part1 = part1(&input);
    println!("Part1: {}", part1);
    let part2 = part2(&input);
    println!("Part2: {}", part2);
}

fn part1(input: &Vec<String>) -> i32 {
    //1. Go through and convert to a vec of ints
    let mut problem_set: Vec<Vec<i32>> = Vec::new();
    for s in input {
        let row: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        problem_set.push(row);
    }
    let mut sum = 0;
    for row in problem_set {
        sum += solve_part_1_row(&row, 0);
    }
    sum
}


fn part2(input: &Vec<String>) -> i32 {
    //1. Go through and convert to a vec of ints
    let mut problem_set: Vec<Vec<i32>> = Vec::new();
    for s in input {
        let row: Vec<i32> = s.split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect();
        problem_set.push(row);
    }
    let mut sum = 0;
    for row in problem_set {
        sum += solve_part_2_row(&row, *row.first().unwrap(), '-');
    }
    sum
}

fn solve_part_1_row(input: &Vec<i32>, sum: i32) -> i32 {
    //Just need to a) keep a running sum b) construct a vector of the differences between each element in input.
    //The base case is going to be all the new elements = 0
    let mut next: Vec<i32> = Vec::new();
    for i in 1..input.len() {
        next.push(input[i] - input[i-1]);
    }
    if next.clone().into_iter().all(|x| x == 0) {
        let new_sum =  sum + input.last().unwrap();
        return new_sum;
    }
    return solve_part_1_row(&next, sum + input.last().unwrap());
}

fn solve_part_2_row(input: &Vec<i32>, sum: i32, operation: char) -> i32 {
    //With this one, we alternate operations and use the first value in each row
    let mut next: Vec<i32> = Vec::new();
    for i in 1..input.len() {
        next.push(input[i] - input[i-1]);
    }
        let mut next_op = '-';
        let mut new_sum = 0;
        if operation == '-' {
            new_sum =  sum - next.first().unwrap();
            next_op = '+';
        } else {
            new_sum = sum + next.first().unwrap();
        }
    if next.clone().into_iter().all(|x| x == 0) {
        return new_sum;
    }
    return solve_part_2_row(&next, new_sum, next_op);
}