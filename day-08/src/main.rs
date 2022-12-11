use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

fn main() {
    let grid = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&grid));
    println!("Part 2: {}", part2(&grid));
}

fn parse_input(filepath: &str) -> Vec<Vec<i32>> {
    read_to_string(filepath).unwrap().lines().map(|line| {
        line.chars().map(|char| char.to_string().parse().unwrap()).collect()
    }).collect()
}

fn part1(grid: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    let w = grid.len();
    let h = grid[0].len();

    for y in 0..h {
        for x in 0..w {
            if x == 0 || y == 0 || y == h - 1 || x == w - 1 {
                answer += 1;
                continue;
            }

            let left = (0..x).all(|i| grid[y][i] < grid[y][x]);
            let up = (0..y).all(|i| grid[i][x] < grid[y][x]);
            let right = (x + 1..w).all(|i| grid[y][i] < grid[y][x]);
            let down = (y + 1..h).all(|i| grid[i][x] < grid[y][x]);

            if left || up || right || down {
                answer += 1;
            }
        }
    }

    answer
}

fn part2(grid: &Vec<Vec<i32>>) -> i32 {
    let mut answer = 0;
    let w = grid.len();
    let h = grid[0].len();

    for y in 0..h {
        for x in 0..w {
            let left = {
                let mut value = 0;
                for i in (0..x).rev() {
                    value += 1;
                    if grid[y][i] >= grid[y][x] {
                        break;
                    }
                }
                value
            };

            let up = {
                let mut value = 0;
                for i in (0..y).rev() {
                    value += 1;
                    if grid[i][x] >= grid[y][x] {
                        break;
                    }
                }
                value
            };

            let right = {
                let mut value = 0;
                for i in x + 1..w {
                    value += 1;
                    if grid[y][i] >= grid[y][x] {
                        break;
                    }
                }
                value
            };

            let down = {
                let mut value = 0;
                for i in y + 1..h {
                    value += 1;
                    if grid[i][x] >= grid[y][x] {
                        break;
                    }
                }
                value
            };

            answer = i32::max(left * up * right * down, answer);
        }
    }

    answer
}
