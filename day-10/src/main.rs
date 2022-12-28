use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn main() {
    let instructions = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&instructions));
    println!("Part 2: ");
    part2(&instructions);
}

fn parse_input(filepath: &str) -> Vec<Instruction> {
    let text = read_to_string(filepath).unwrap();
    let lines = text.lines();
    let mut instructions = vec![];

    for line in lines {
        let mut info = line.split(" ");
        let command = info.next().unwrap();

        if command == "addx" {
            let number = info.next().unwrap().parse().unwrap();
            instructions.push(Instruction::Noop);
            instructions.push(Instruction::Addx(number));
        }
        else if command == "noop" {
            instructions.push(Instruction::Noop);
        }
        else {
            panic!("Should never happen")
        }
    }

    instructions
}

fn part1(instructions: &Vec<Instruction>) -> i32 {
    let mut check_cycle = 60;
    let mut x = 1;
    let mut answer = 0;
    let mut sum_before_20 = 1;
    let mut instructions = instructions.iter();

    for cycle in 2..=220 {
        match instructions.next().unwrap() {
            Instruction::Addx(num) => {
                x += num;
                if cycle <= 20 {
                    sum_before_20 += num
                }
            }
            Instruction::Noop => {}
        }

        if cycle == 20 {
            answer += sum_before_20 * 20;
        }
        else if cycle == check_cycle {
            answer += cycle * x;
            check_cycle += 40;
        }
    }

    answer
}

fn part2(instructions: &Vec<Instruction>) {
    let mut x = 1;
    let mut instructions = instructions.iter();
    let mut row = 0;

    let fill_char = '#';
    let empty_char = '.';

    print!("{}", fill_char);
    for cycle in 2..=240 {
        match instructions.next().unwrap() {
            Instruction::Addx(num) => x += num,
            Instruction::Noop => {}
        }

        let spr_pos = cycle - (row * 40);
        if spr_pos == x || spr_pos == x + 1 || spr_pos == x + 2 {
            print!("{}", fill_char);
        }
        else {
            print!("{}", empty_char);
        }

        if cycle % 40 == 0 {
            print!("\n");
            row += 1;
        }
    }
}
