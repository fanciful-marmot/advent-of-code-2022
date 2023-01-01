use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

enum Command {
    CD(String),
    LS,
}

struct LsResult {
    identifier: String,
    size: u64,
}

impl LsResult {
    fn from_line(line: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d+|dir) ([\w\.]+)$").unwrap();
        }

        let captures = RE.captures(line).unwrap();

        LsResult {
            identifier: String::from(captures.get(2).unwrap().as_str()),
            size: u64::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap_or_default(),
        }
    }
}

struct FileSystemNode {
    identifier: String,
    children: Vec<usize>,    // Indexes of the child nodes in the node array
    size: u64,               // Size of this particular node
    total_size: Option<u64>, // Size of this node plus total size of all children
    parent: Option<usize>,   // Index of the parent node in the node array
}

impl FileSystemNode {
    fn with_identifier(name: &str) -> Self {
        FileSystemNode {
            identifier: String::from(name),
            children: Vec::new(),
            size: 0,
            total_size: None,
            parent: None,
        }
    }

    fn add_child(&mut self, child_index: usize) {
        self.children.push(child_index)
    }

    fn is_dir(&self) -> bool {
        self.children.len() > 0
    }
}

struct FileSystem {
    nodes: Vec<FileSystemNode>, // Root is always index 0
}

impl FileSystem {
    fn new() -> Self {
        FileSystem {
            nodes: vec![FileSystemNode::with_identifier("/")],
        }
    }

    fn get_node(&self, current_node: usize, name: &str) -> Option<usize> {
        let node = &self.nodes[current_node];

        if name == ".." {
            node.parent
        } else {
            node.children
                .iter()
                .find(|&node_index| self.nodes[*node_index].identifier == name)
                .map(|&index| index)
        }
    }

    fn add_child(&mut self, current_node: usize, child: FileSystemNode) {
        let child_index = self.nodes.len();
        self.nodes[current_node].add_child(child_index);

        let mut child = child;
        child.parent = Some(current_node);

        self.nodes.push(child);
    }

    fn compute_total_size(&mut self, working_node: usize) -> u64 {
        let node = &self.nodes[working_node];

        if let Some(total_size) = node.total_size {
            return total_size;
        }

        let mut total_size = node.size;
        let child_indices = node.children.clone();

        for child_index in child_indices.iter() {
            total_size += self.compute_total_size(*child_index);
        }

        // Cache it
        self.nodes[working_node].total_size = Some(total_size);

        total_size
    }

    // fn print(&self) {
    //     for (index, node) in self.nodes.iter().enumerate() {
    //         println!(
    //             "{}: '{}', {:?}, {:?}, {}, {:?}",
    //             index, node.identifier, node.parent, node.children, node.size, node.total_size
    //         );
    //     }
    // }
}

fn get_command(line: &str) -> Command {
    if line.starts_with("$ ls") {
        Command::LS
    } else if line.starts_with("$ cd") {
        Command::CD(String::from(&line[5..]))
    } else {
        panic!("Failed to parse command from line {}", line);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let file_system = read_input(filename);

    println!("part 1: {}", part1(&file_system));
    println!("part 2: {}", part2(&file_system));
}

fn read_input(filename: &str) -> FileSystem {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let mut lines = contents.lines().skip(1); // Skip the first '/' line

    let mut file_system = FileSystem::new();
    let mut working_node_index: usize = 0;

    while let Some(line) = lines.next() {
        match get_command(line) {
            Command::CD(name) => {
                if let Some(new_index) = file_system.get_node(working_node_index, &name) {
                    working_node_index = new_index;
                } else {
                    panic!("CD'd into unknown directory '{}'", name);
                }
            }
            Command::LS => {
                for ls_line in lines.clone().take_while(|line| !line.starts_with("$")) {
                    let ls = LsResult::from_line(ls_line);

                    let mut new_node = FileSystemNode::with_identifier(&ls.identifier);
                    new_node.size = ls.size;

                    file_system.add_child(working_node_index, new_node);

                    lines.next(); // Skip line in the main iterator
                }
            }
        }
    }

    file_system.compute_total_size(0);

    file_system
}

fn part1(root: &FileSystem) -> u64 {
    root.nodes
        .iter()
        .filter(|&node| node.is_dir())
        .filter(|&node| {
            if let Some(total_size) = node.total_size {
                total_size <= 100000
            } else {
                false
            }
        })
        .map(|node| node.total_size.unwrap_or_default())
        .sum()
}

fn part2(root: &FileSystem) -> u64 {
    static TOTAL_CAPACITY: u64 = 70000000;
    static REQUIRED_SPACE: u64 = 30000000;
    static MAX_TREE_SIZE: u64 = TOTAL_CAPACITY - REQUIRED_SPACE;

    let current_size = root.nodes[0].total_size.unwrap();
    let required_deletion = current_size - MAX_TREE_SIZE; // Must find directory as close to this as possible

    root.nodes
        .iter()
        .filter(|&node| node.is_dir())
        .filter(|&node| {
            if let Some(total_size) = node.total_size {
                total_size >= required_deletion
            } else {
                false
            }
        })
        .map(|node| node.total_size.unwrap_or_default())
        .min()
        .unwrap_or_default()
}
