use std::fs::read_to_string;
use std::fmt::{self, Display};

const INPUT_FILEPATH: &str = "./input.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Data {
    Num(i32),
    List(Vec<Data>),
}

impl Display for Data {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Data::Num(num) => write!(f, "{}", num.to_string()),
            Data::List(list) => {
                let text = list
                    .iter()
                    .map(Data::to_string)
                    .collect::<Vec<String>>()
                    .join(", ");
                write!(f, "[{}]", text)
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum CmpResult {
    Correct,
    Wrong,
    WhoKnows,
}

fn main() {
    let signals = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(signals.clone()));
    println!("Part 2: {}", part2(signals));
}

fn parse_data(data: &mut String) -> Data {
    let mut first = data.remove(0).to_string();

    if first.parse::<i32>().is_ok() {
        loop {
            let char = data.remove(0);
            if !char.is_alphanumeric() {
                data.insert(0, char);
                break;
            }
            first.push(char);
        }
        Data::Num(first.parse::<i32>().unwrap())
    }
    else if first == "[" {
        let mut items = vec![];
        loop {
            let char = data.remove(0);

            if char == ']' { break; }
            if char == ',' { continue; }

            data.insert(0, char);
            let item = parse_data(data);
            items.push(item);
        }
        Data::List(items)
    }
    else {
        panic!("Unexpected char: `{}`", first);
    }
}

fn parse_input(filepath: &str) -> Vec<Data> {
    let text = read_to_string(filepath).unwrap();

    let signals: Vec<Data> = text
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| parse_data(&mut line.to_string()))
        .collect();

    signals
}

fn cmp_order(part1: &Data, part2: &Data) -> CmpResult {
    if let (Data::Num(num1), Data::Num(num2)) = (part1, part2) {
        return if num1 < num2 {
            CmpResult::Correct
        } else if num1 > num2 {
            CmpResult::Wrong
        } else {
            CmpResult::WhoKnows
        };
    }

    let list1 = if let Data::Num(_) = part1 {
        vec![part1.clone()]
    } else if let Data::List(list1) = part1 {
        list1.clone()
    } else {
        unreachable!()
    };

    let list2 = if let Data::Num(_) = part2 {
        vec![part2.clone()]
    } else if let Data::List(list2) = part2 {
        list2.clone()
    } else {
        unreachable!()
    };

    for (item1, item2) in list1.iter().zip(&list2) {
        let result = cmp_order(item1, item2);
        if result != CmpResult::WhoKnows {
            return result;
        }
    }

    return if list1.len() < list2.len() {
        CmpResult::Correct
    } else if list1.len() > list2.len() {
        CmpResult::Wrong
    } else {
        CmpResult::WhoKnows
    };
}

fn part1(mut signals: Vec<Data>) -> i32 {
    let mut answer = 0;
    let mut index = 1;

    while !signals.is_empty() {
        let part1 = signals.remove(0);
        let part2 = signals.remove(0);

        if cmp_order(&part1, &part2) == CmpResult::Correct {
            answer += index;
        }
        index += 1;
    }

    answer
}

fn sort_signals(array: &mut [Data]) {
    let n = array.len();
    if n < 2 {
        return;
    }
    let pivot = partition(array);
    sort_signals(&mut array[..pivot]);
    sort_signals(&mut array[pivot + 1..]);
}

fn partition(array: &mut [Data]) -> usize {
    let n = array.len();
    let pivot_index = n / 2;
    array.swap(pivot_index, n - 1);
    let mut store_index = 0;
    for i in 0..n - 1 {
        if cmp_order(&array[i], &array[n - 1]) == CmpResult::Correct {
            array.swap(i, store_index);
            store_index += 1;
        }
    }
    array.swap(store_index, n - 1);
    store_index
}

fn part2(mut signals: Vec<Data>) -> usize {
    let el1 = Data::List(vec![Data::List(vec![Data::Num(2)])]);
    let el2 = Data::List(vec![Data::List(vec![Data::Num(6)])]);

    signals.push(el1.clone());
    signals.push(el2.clone());

    sort_signals(&mut signals);

    let mut index1 = 0;
    let mut index2 = 0;

    for (i, signal) in signals.iter().enumerate() {
        if signal == &el1 {
            index1 = i + 1;
        } else if signal == &el2 {
            index2 = i + 1;
        }
    }

    index1 * index2
}
