use std::fs::read_to_string;
use std::collections::HashSet;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug)]
enum Dir {
    Up,
    Down,
    Right,
    Left,
}

impl Dir {
    fn from_str(str: &str) -> Option<Self> {
        match str {
            "U" => Some(Self::Up),
            "D" => Some(Self::Down),
            "R" => Some(Self::Right),
            "L" => Some(Self::Left),
            _ => None,
        }
    }

    fn to_vec(&self) -> Vec2 {
        match self {
            Dir::Up => Vec2 { x: 0, y: -1 },
            Dir::Down => Vec2 { x: 0, y: 1 },
            Dir::Right => Vec2 { x: 1, y: 0 },
            Dir::Left => Vec2 { x: -1, y: 0 },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn direction_to(&self, other: &Vec2) -> Vec2 {
        let mut dir = Vec2 { x: 0, y: 0 };
        if other.x > self.x { dir.x += 1; }
        if other.x < self.x { dir.x -= 1; }
        if other.y > self.y { dir.y += 1; }
        if other.y < self.y { dir.y -= 1; }
        dir
    }

    fn move_towards(&mut self, direction: &Vec2) {
        self.x += direction.x;
        self.y += direction.y;
    }
}

fn main() {
    let instructions = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: {}", part2(&instructions));
}

fn parse_input(filepath: &str) -> Vec<(Dir, i32)> {
    read_to_string(filepath).unwrap().lines().map(|line| {
        let mut info = line.split(" ");
        let dir = info.next().unwrap();
        let num = info.next().unwrap().parse().unwrap();
        (Dir::from_str(dir).unwrap(), num)
    }).collect()
}

fn part1(instructions: &Vec<(Dir, i32)>) -> i32 {
    let mut head = Vec2 { x: 0, y: 0 };
    let mut tail = Vec2 { x: 0, y: 0 };
    let mut visited: HashSet<Vec2> = HashSet::new();

    for (dir, amount) in instructions {
        let dir = dir.to_vec();
        for _ in 0..*amount {
            head.move_towards(&dir);
            if i32::abs(head.x - tail.x) >= 2 || i32::abs(head.y - tail.y) >= 2 {
                tail.move_towards(&tail.direction_to(&head));
            }
            visited.insert(tail.clone());
        }
    }

    visited.len() as i32
}

fn part2(instructions: &Vec<(Dir, i32)>) -> i32 {
    let mut body = vec![Vec2 { x: 0, y: 0 }; 10];
    let mut visited: HashSet<Vec2> = HashSet::new();

    for (dir, amount) in instructions {
        let dir = dir.to_vec();
        for _ in 0..*amount {
            body[0].move_towards(&dir);
            for i in 1..body.len() {
                let tail_index = body.len() - 1;
                let head = body[i - 1].clone();
                let tail = &mut body[i];

                if i32::abs(head.x - tail.x) >= 2 || i32::abs(head.y - tail.y) >= 2 {
                    tail.move_towards(&tail.direction_to(&head));
                }

                if i == tail_index {
                    visited.insert(tail.clone());
                }
            }
        }
    }

    visited.len() as i32
}
