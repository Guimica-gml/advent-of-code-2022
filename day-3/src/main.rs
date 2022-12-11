use std::{fs::read_to_string, collections::HashSet};

const INPUT_FILEPATH: &str = "./input.txt";

fn main() {
    let rucksacks = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
}

fn parse_input(filepath: &str) -> Vec<String> {
    read_to_string(filepath).unwrap().lines().map(|line| line.to_string()).collect()
}

fn get_char_score(char: char) -> i32 {
    if char.is_uppercase() {
        (char as i32) - 38
    }
    else {
        (char as i32) - 96
    }
}

fn part1(rucksacks: &Vec<String>) -> i32 {
    let mut answer = 0;

    for rucksack in rucksacks {
        let split_index = rucksack.len() / 2;
        let (first, second) = rucksack.split_at(split_index);

        let first: HashSet<char> = HashSet::from_iter(first.chars());
        let second: HashSet<char> = HashSet::from_iter(second.chars());

        let mut common_chars = first.intersection(&second);
        let char = *common_chars.next().unwrap();
        answer += get_char_score(char);
    }

    answer
}

fn part2(rucksacks: &Vec<String>) -> i32 {
    let mut answer = 0;
    let mut iter = rucksacks.into_iter();

    loop {
        let first = match iter.next() {
            Some(v) => v,
            None => break,
        };
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();

        let first: HashSet<char> = HashSet::from_iter(first.chars());
        let second: HashSet<char> = HashSet::from_iter(second.chars());

        let mut third: HashSet<char> = HashSet::from_iter(third.chars());
        third = HashSet::from_iter(third.intersection(&second).into_iter().copied());

        let mut common_chars = first.intersection(&third);
        let char = *common_chars.next().unwrap();
        answer += get_char_score(char);
    }

    answer
}
