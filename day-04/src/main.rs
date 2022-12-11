use std::collections::HashSet;
use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

fn main() {
    let pair_paths = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&pair_paths));
    println!("Part 2: {}", part2(&pair_paths));
}

fn parse_input(filepath: &str) -> Vec<(HashSet<i32>, HashSet<i32>)> {
    read_to_string(filepath).unwrap().lines().map(|line| {
        let mut paths = line.split(",");

        let mut first_path = paths.next().unwrap().split("-");
        let start: i32 = first_path.next().unwrap().parse().unwrap();
        let end: i32 = first_path.next().unwrap().parse().unwrap();
        let first_range = HashSet::from_iter(start..end + 1);

        let mut second_path = paths.next().unwrap().split("-");
        let start: i32 = second_path.next().unwrap().parse().unwrap();
        let end: i32 = second_path.next().unwrap().parse().unwrap();
        let second_range = HashSet::from_iter(start..end + 1);

        (first_range, second_range)
    }).collect()
}

fn part1(pair_paths: &Vec<(HashSet<i32>, HashSet<i32>)>) -> i32 {
    let mut answer = 0;

    for (first_path, second_path) in pair_paths {
        if first_path.is_subset(second_path) || second_path.is_subset(first_path) {
            answer += 1;
        }
    }

    answer
}

fn part2(pair_paths: &Vec<(HashSet<i32>, HashSet<i32>)>) -> i32 {
    let mut answer = 0;

    for (first_path, second_path) in pair_paths {
        if !first_path.is_disjoint(second_path) || !second_path.is_disjoint(first_path) {
            answer += 1;
        }
    }

    answer
}
