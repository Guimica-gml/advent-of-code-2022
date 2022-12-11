use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

fn main() {
    let calories_list = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&calories_list));
    println!("Part 2: {}", part2(&calories_list));
}

fn parse_input(filepath: &str) -> Vec<i32> {
    read_to_string(filepath).unwrap().split("\n\n").map(|chunk| {
        chunk.lines().map(|line| line.parse::<i32>().unwrap()).sum()
    }).collect()
}

fn part1(calories_list: &Vec<i32>) -> i32 {
    *calories_list.iter().max().unwrap()
}

fn part2(calories_list: &Vec<i32>) -> i32 {
    let mut sorted = calories_list.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    sorted[0..3].iter().sum()
}
