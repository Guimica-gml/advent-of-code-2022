use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

#[allow(dead_code)]
#[derive(Debug)]
struct Instruction {
    amount: usize,
    from: usize,
    to: usize,
}

fn main() {
    let (stacks, instructions) = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(stacks.clone(), &instructions));
    println!("Part 2: {}", part2(stacks.clone(), &instructions));
}

fn parse_input(filepath: &str) -> (Vec<Vec<char>>, Vec<Instruction>) {
    let text = read_to_string(filepath).unwrap();
    let mut blocks = text.split("\n\n");

    let stacks_text = blocks.next().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![];

    for line in stacks_text.lines().rev().skip(1) {
        let mut chars = line.chars();
        let mut index = 0;

        loop {
            let char = chars.nth(1).unwrap();

            if char != ' ' {
                if stacks.len() < index + 1 {
                    stacks.push(vec![char]);
                }
                else {
                    stacks.get_mut(index).unwrap().push(char);
                }
            }

            index += 1;
            match chars.nth(1) {
                None => break,
                _ => {}
            }
        }
    }

    let instructions_text = blocks.next().unwrap();
    let instructions = instructions_text.lines().map(|line| {
        let mut info = line.split(" ");
        let amount = info.nth(1).unwrap().parse().unwrap();
        let from = info.nth(1).unwrap().parse().unwrap();
        let to = info.nth(1).unwrap().parse().unwrap();
        Instruction { amount, from, to }
    }).collect();

    (stacks, instructions)
}

fn part1(mut stacks: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let Instruction { amount, from, to } = *instruction;
        for _ in 0..amount {
            let stack = stacks.get_mut(from - 1).unwrap();
            let crate_char = stack.remove(stack.len() - 1);
            stacks.get_mut(to - 1).unwrap().push(crate_char);
        }
    }

    let mut answer = String::new();

    for stack in stacks {
        answer += stack.iter().last().unwrap().to_string().as_str();
    }

    answer
}

fn part2(mut stacks: Vec<Vec<char>>, instructions: &Vec<Instruction>) -> String {
    for instruction in instructions {
        let Instruction { amount, from, to } = *instruction;
        let mut temp = vec![];
        for _ in 0..amount {
            let stack = stacks.get_mut(from - 1).unwrap();
            let crate_char = stack.remove(stack.len() - 1);
            temp.insert(0, crate_char);
        }
        stacks.get_mut(to - 1).unwrap().append(&mut temp);
    }

    let mut answer = String::new();

    for stack in stacks {
        answer += stack.iter().last().unwrap().to_string().as_str();
    }

    answer
}
