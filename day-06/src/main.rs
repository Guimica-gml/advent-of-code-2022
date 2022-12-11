use std::fs::read_to_string;
use std::collections::HashSet;

const INPUT_FILEPATH: &str = "./input.txt";

fn main() {
    let message = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&message));
    println!("Part 2: {}", part2(&message));
}

fn parse_input(filepath: &str) -> String {
    read_to_string(filepath).unwrap()
}

fn repeats_chars(vec: &Vec<char>) -> bool {
    HashSet::<char>::from_iter(vec.iter().copied()).len() < vec.len()
}

fn index_of_substring(message: &String, size: usize) -> i32 {
    let mut index = 0;
    let mut memory: Vec<char> = vec![];

    for char in message.chars() {
        index += 1;

        memory.push(char);
        if memory.len() <= size {
            continue;
        }

        memory.remove(0);
        if !repeats_chars(&memory) {
            break;
        }
    }

    index
}

fn part1(message: &String) -> i32 {
    index_of_substring(message, 4)
}

fn part2(message: &String) -> i32 {
    index_of_substring(message, 14)
}
