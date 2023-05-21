use std::fs::read_to_string;
use std::collections::HashSet;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone)]
struct Map {
    walls: HashSet<Coord>,
    top_left: Coord,
    bottom_right: Coord,
}

fn main() {
    let map = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&map));
    println!("Part 2: {}", part2(&map));
}

fn parse_input(filepath: &str) -> Map {
    let text = read_to_string(filepath).unwrap();
    let mut walls = HashSet::<Coord>::new();

    let lines = text.lines();
    for line in lines {
        let coords_text = line.split(" -> ");
        let mut boundaries = vec![];

        for coord_text in coords_text {
            let a = coord_text.split_once(",").unwrap();
            let x = a.0.parse::<i32>().unwrap();
            let y = a.1.parse::<i32>().unwrap();

            let coord = Coord { x, y };
            boundaries.push(coord);
        }

        let mut start = boundaries.remove(0);

        for boundary in boundaries {
            while start != boundary {
                walls.insert(start);
                start.x += i32::signum(boundary.x - start.x);
                start.y += i32::signum(boundary.y - start.y);
            }
        }

        walls.insert(start);
    }

    let left = walls.iter().map(|p| p.x).min().unwrap();
    let right = walls.iter().map(|p| p.x).max().unwrap();
    let bottom = walls.iter().map(|p| p.y).max().unwrap();

    let top_left = Coord { x: left, y: 0 };
    let bottom_right = Coord { x: right, y: bottom };

    Map { walls, top_left, bottom_right }
}

#[allow(dead_code)]
fn draw_map(map: &Map, balls: &HashSet<Coord>) {
    for y in map.top_left.y..=map.bottom_right.y+1 {
        for x in map.top_left.x..=map.bottom_right.x {
            let coord = Coord { x, y };
            if map.walls.contains(&coord) {
                print!("#");
            } else if balls.contains(&coord) {
                print!("o");
            } else if coord == (Coord { x: 500, y: 0 }) {
                print!("+");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }

    let count = (map.bottom_right.x - map.top_left.x + 1) as usize;
    println!("{:-<1$}", "", count);
}

fn is_occupied(map: &Map, balls: &HashSet<Coord>, x: i32, y: i32) -> bool {
    let coord = Coord { x, y };
    map.walls.contains(&coord) || balls.contains(&coord)
}

fn part1(map: &Map) -> usize {
    let mut balls = HashSet::<Coord>::new();

    'outer: loop {
        let mut ball = Coord { x: 500, y: 0 };

        'redo: loop {
            // Make ball fall to the ground
            while !is_occupied(map, &balls, ball.x, ball.y + 1) {
                if ball.y >= map.bottom_right.y {
                    break 'outer;
                }
                ball.y += 1;
            }

            // Check if it can slide
            for i in -1..=1 {
                let mut pos = ball;
                pos.x += i;
                pos.y += 1;

                if !is_occupied(map, &balls, pos.x, pos.y) {
                    ball = pos;
                    continue 'redo;
                }
            }

            break 'redo;
        }

        balls.insert(ball);
    }

    balls.len()
}

fn part2(map: &Map) -> usize {
    let mut balls = HashSet::<Coord>::new();

    'outer: loop {
        let mut ball = Coord { x: 500, y: 0 };

        'redo: loop {
            // Make ball fall to the ground
            while !is_occupied(map, &balls, ball.x, ball.y + 1) {
                if ball.y >= map.bottom_right.y + 1 {
                    break 'redo;
                }
                ball.y += 1;
            }

            // Check if it can slide
            for i in -1..=1 {
                let mut pos = ball;
                pos.x += i;
                pos.y += 1;

                if !is_occupied(map, &balls, pos.x, pos.y) {
                    ball = pos;
                    continue 'redo;
                }
            }

            break 'redo;
        }

        balls.insert(ball);
        if ball == (Coord { x: 500, y: 0}) {
            break 'outer;
        }
    }

    balls.len()
}
