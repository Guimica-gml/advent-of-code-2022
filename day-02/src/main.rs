use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, PartialEq, Eq)]
enum MatchResult {
    Win,
    Loss,
    Draw,
}

impl MatchResult {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "X" => Some(Self::Loss),
            "Y" => Some(Self::Draw),
            "Z" => Some(Self::Win),
            _ => None
        }
    }
}

#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "A" | "X" => Some(Self::Rock),
            "B" | "Y" => Some(Self::Paper),
            "C" | "Z" => Some(Self::Scissor),
            _ => None
        }
    }

    fn score(&self) -> i32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }

    fn result_against(&self, other: &Shape) -> MatchResult {
        match self {
            Shape::Rock => match other {
                Shape::Rock => MatchResult::Draw,
                Shape::Paper => MatchResult::Loss,
                Shape::Scissor => MatchResult::Win,
            },
            Shape::Paper => match other {
                Shape::Rock => MatchResult::Win,
                Shape::Paper => MatchResult::Draw,
                Shape::Scissor => MatchResult::Loss,
            },
            Shape::Scissor => match other {
                Shape::Rock => MatchResult::Loss,
                Shape::Paper => MatchResult::Win,
                Shape::Scissor => MatchResult::Draw,
            },
        }
    }
}

fn main() {
    let plays = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&plays));
    println!("Part 2: {}", part2(&plays));
}

fn parse_input(filepath: &str) -> Vec<(String, String)> {
    read_to_string(filepath).unwrap().lines().map(|line| {
        let mut plays = line.split(" ");
        let first = plays.next().unwrap();
        let second = plays.next().unwrap();
        (first.to_string(), second.to_string())
    }).collect()
}

fn get_shape_to_achive_result(wanted_result: &MatchResult, oponent_shape: &Shape) -> Shape {
    match wanted_result {
        MatchResult::Win => match oponent_shape {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissor,
            Shape::Scissor => Shape::Rock,
        }
        MatchResult::Loss => match oponent_shape {
            Shape::Rock => Shape::Scissor,
            Shape::Paper => Shape::Rock,
            Shape::Scissor => Shape::Paper,
        }
        MatchResult::Draw => oponent_shape.clone(),
    }
}

fn part1(plays: &Vec<(String, String)>) -> i32 {
    let mut answer = 0;

    for play in plays {
        let oponent_shape = Shape::from_str(play.0.as_str()).unwrap();
        let my_shape = Shape::from_str(play.1.as_str()).unwrap();

        answer += my_shape.score();
        answer += match my_shape.result_against(&oponent_shape) {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        };
    }

    answer
}

fn part2(plays: &Vec<(String, String)>) -> i32 {
    let mut answer = 0;

    for play in plays {
        let oponent_shape = Shape::from_str(play.0.as_str()).unwrap();
        let wanted_result = MatchResult::from_str(play.1.as_str()).unwrap();
        let my_shape = get_shape_to_achive_result(&wanted_result, &oponent_shape);

        answer += my_shape.score();
        answer += match wanted_result {
            MatchResult::Win => 6,
            MatchResult::Loss => 0,
            MatchResult::Draw => 3,
        };
    }

    answer
}
