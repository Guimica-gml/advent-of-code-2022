use std::path::Path;
use std::process::Command;
use std::io::{self, Write};
use std::collections::HashMap;
use std::fs::{read_to_string, OpenOptions, remove_file, File};

const INPUT_FILEPATH: &str = "./input.txt";
const GRAPH_NAME: &str = "computer_input";

#[derive(Debug, Clone)]
enum Item {
    Dir(String, HashMap<String, usize>),
    File(String, u32),
}

impl Item {
    fn new_dir(name: String, parent_index: usize) -> Self {
        Self::Dir(
            name,
            HashMap::from([
                ("/".to_string(), 0),
                ("..".to_string(), parent_index),
            ])
        )
    }

    fn new_file(name: String, size: u32) -> Item {
        Self::File(name, size)
    }
}

#[derive(Debug, Clone)]
struct Computer {
    dir_index: usize,
    filesystem: Vec<Item>,
}

impl Computer {
    fn new() -> Self {
        Self {
            dir_index: 0,
            filesystem: vec![Item::new_dir("/".to_string(), 0)],
        }
    }

    fn change_dir(&mut self, path: &str) {
        match &self.filesystem[self.dir_index] {
            Item::Dir(_, items) => {
                self.dir_index = items[path];
            }
            Item::File(_, _) => panic!("Should never happen"),
        }
    }

    fn add_dir_to_current_path(&mut self, name: String) {
        self.filesystem.push(Item::new_dir(name.clone(), self.dir_index));
        let added_dir_index = self.filesystem.len() - 1;

        match &mut self.filesystem[self.dir_index] {
            Item::Dir(_, items) => {
                items.insert(name, added_dir_index);
            }
            Item::File(_, _) => panic!("Should never happen"),
        }
    }

    fn add_file_to_current_path(&mut self, name: String, size: u32) {
        self.filesystem.push(Item::new_file(name.clone(), size));
        let added_dir_index = self.filesystem.len() - 1;

        match &mut self.filesystem[self.dir_index] {
            Item::Dir(_, items) => {
                items.insert(name, added_dir_index);
            }
            Item::File(_, _) => panic!("Should never happen"),
        }
    }

    fn get_dir_size(&self, sizes: &mut Vec<u32>, index: usize) -> u32 {
        match &self.filesystem[index] {
            Item::Dir(_, items) => {
                let mut dir_size = 0;
                for (name, item) in items {
                    if let Item::File(_, size) = &self.filesystem[*item] {
                        dir_size += size
                    }
                    else if name != ".." && name != "/" {
                        dir_size += self.get_dir_size(sizes, *item);
                    }
                }
                sizes.push(dir_size);
                dir_size
            }
            Item::File(_, _) => panic!("This Should never happen"),
        }
    }

    fn get_all_dir_sizes(&self) -> Vec<u32> {
        let mut sizes = vec![];
        self.get_dir_size(&mut sizes, 0);
        sizes
    }

    fn write_dir_to_graph(&self, file: &mut File, index: usize) -> Result<(), io::Error> {
        match &self.filesystem[index] {
            Item::Dir(_, items) => {
                for (name, item) in items {
                    if name != ".." && name != "/" {
                        self.write_dir_to_graph(file, *item)?;
                        writeln!(file, "    {} -- {};", index, item)?;
                    }
                }
            }
            Item::File(_, _) => {}
        }

        Ok(())
    }

    fn generate_graph(&self, dir: &str, name: &str) -> Result<(), io::Error> {
        let dot_path = Path::new(dir).join(format!("{}.dot", name));
        let svg_path = Path::new(dir).join(format!("{}.svg", name));
        let mut file = OpenOptions::new().write(true).create(true).open(dot_path.clone())?;

        writeln!(&mut file, "graph {} {{", name)?;

        for (i, item) in self.filesystem.iter().enumerate() {
            match item {
                Item::Dir(name, _) => writeln!(&mut file, "    {} [label=\"{} (dir)\"];", i, name)?,
                Item::File(name, size) => writeln!(&mut file, "    {} [label=\"{} (file)\n[{}]\"];", i, name, size)?,
            }
        }

        writeln!(&mut file, "    graph [ordering=\"out\", overlap=false, splines=false];")?;
        writeln!(&mut file, "    graph [pad=\"0.5\", nodesep=\"0.5\", ranksep=\"1\"];")?;
        self.write_dir_to_graph(&mut file, 0)?;
        writeln!(&mut file, "}}")?;

        _ = Command::new("dot")
            .arg("-Tsvg")
            .arg(dot_path.as_os_str())
            .arg("-o")
            .arg(svg_path.as_os_str())
            .output()?;

        remove_file(dot_path.as_os_str())?;
        Ok(())
    }
}

fn main() {
    let computer = parse_input(INPUT_FILEPATH);
    println!("Part 1: {}", part1(&computer));
    println!("Part 2: {}", part2(&computer));

    match computer.generate_graph("./graph/", GRAPH_NAME) {
        Ok(_) => {}
        Err(err) => eprintln!("Error: failed to generate graph: {}", err),
    }
}

fn parse_input(filepath: &str) -> Computer {
    let text = read_to_string(filepath).unwrap();
    let lines = text.lines();
    let mut computer = Computer::new();

    for line in lines {
        let mut parts = line.split(" ");
        let header = parts.next().unwrap();

        if header == "$" {
            let command = parts.next().unwrap();
            if command == "cd" {
                let new_path = parts.next().unwrap();
                computer.change_dir(new_path);
            }
        }
        else if header == "dir" {
            let dir_name = parts.next().unwrap();
            computer.add_dir_to_current_path(dir_name.to_string());
        }
        else if header.parse::<u32>().is_ok() {
            let file_name = parts.next().unwrap();
            let size = header.parse::<u32>().unwrap();
            computer.add_file_to_current_path(file_name.to_string(), size);
        }
    }

    computer
}

fn part1(computer: &Computer) -> u32 {
    let sizes = computer.get_all_dir_sizes();
    sizes.into_iter().filter(|size| *size < 100_000).sum()
}

fn part2(computer: &Computer) -> u32 {
    let mut sizes = computer.get_all_dir_sizes();
    sizes.sort();

    let total_space = 70_000_000;
    let required_space = 30_000_000;
    let used_space = *sizes.iter().last().unwrap();
    let availabe_space = total_space - used_space;
    let space_to_free = required_space - availabe_space;

    sizes.into_iter().find(|size| *size > space_to_free).unwrap()
}
