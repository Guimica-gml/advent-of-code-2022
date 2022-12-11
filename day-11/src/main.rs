use std::fs::read_to_string;

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i64>,
    operation: String,
    test: i64,
    throw_index_if_true: usize,
    throw_index_if_false: usize,
}

fn main() {
    let monkeys = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(monkeys.clone()));
    println!("Part 2: {}", part2(monkeys.clone()));
}

fn parse_input(filepath: &str) -> Vec<Monkey> {
    read_to_string(filepath).unwrap().split("\n\n").map(|block| {
        let mut lines = block.lines().skip(1);

        let items = lines.next().unwrap().split(": ").nth(1).unwrap().split(", ").map(|text| {
            text.parse().unwrap()
        }).collect();

        let operation = lines.next().unwrap().split(" = ").last().unwrap().to_string();
        let test = lines.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();
        let throw_index_if_true = lines.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();
        let throw_index_if_false = lines.next().unwrap().trim().split(" ").last().unwrap().parse().unwrap();

        Monkey { items, operation, test, throw_index_if_true, throw_index_if_false }
    }).collect()
}

fn simulate_monkeys(mut monkeys: Vec<Monkey>, rounds: usize, is_super_worried: bool) -> i64 {
    let mut monkey_activity = vec![0; monkeys.len()];

    let mut modulo = 1;
    for monkey in &monkeys {
        modulo *= monkey.test;
    }

    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while monkeys[i].items.len() > 0 {
                let item = monkeys[i].items.remove(0);

                let mut op_info = monkeys[i].operation.split(" ");
                let op_left: i64 = match op_info.next().unwrap().parse() {
                    Ok(v) => v,
                    Err(_) => item,
                };
                let op = op_info.next().unwrap();
                let op_right: i64 = match op_info.next().unwrap().parse() {
                    Ok(v) => v,
                    Err(_) => item,
                };

                let mut new_worry_level = match op {
                    "*" => (op_left * op_right) % modulo,
                    "+" => (op_left + op_right) % modulo,
                    _ => panic!("Unexpected operation: {}", op),
                };

                if !is_super_worried {
                    new_worry_level /= 3;
                }

                let if_true = monkeys[i].throw_index_if_true;
                let if_false = monkeys[i].throw_index_if_false;

                if new_worry_level % monkeys[i].test == 0 {
                    monkeys[if_true].items.push(new_worry_level);
                }
                else {
                    monkeys[if_false].items.push(new_worry_level);
                }

                monkey_activity[i] += 1;
            }
        }
    }

    monkey_activity.sort_by(|a, b| b.cmp(a));
    monkey_activity[0] * monkey_activity[1]
}

fn part1(monkeys: Vec<Monkey>) -> i64 {
    simulate_monkeys(monkeys, 20, false)
}

fn part2(monkeys: Vec<Monkey>) -> i64 {
    simulate_monkeys(monkeys, 10_000, true)
}
