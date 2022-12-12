use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::fs;

struct MoveOperation {
    n: usize,
    from: usize,
    to: usize,
}

impl MoveOperation {
    pub fn from_str(s: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }

        let captures = RE.captures(s).unwrap();

        MoveOperation {
            n: usize::from_str_radix(captures.get(1).unwrap().as_str(), 10).unwrap(),
            from: usize::from_str_radix(captures.get(2).unwrap().as_str(), 10).unwrap(),
            to: usize::from_str_radix(captures.get(3).unwrap().as_str(), 10).unwrap(),
        }
    }
}

#[derive(Clone)]
struct Cargo {
    lanes: Vec<Vec<char>>,
}

impl Cargo {
    pub fn with_lanes(n: usize) -> Self {
        let mut lanes = Vec::with_capacity(n);

        for _ in 0..n {
            lanes.push(Vec::new());
        }

        Cargo { lanes }
    }

    pub fn push_to_lane(&mut self, lane: usize, item: char) {
        self.lanes[lane].push(item);
    }

    pub fn move_items(&mut self, n: usize, from: usize, to: usize) {
        if self.lanes[from].len() < n {
            panic!(
                "Tried to move {} from lane {} when it only has {} items",
                n,
                from,
                self.lanes[from].len()
            );
        }

        for _ in 0..n {
            let item = self.lanes[from].pop().unwrap();
            self.lanes[to].push(item);
        }
    }

    pub fn move_items_batch(&mut self, n: usize, from: usize, to: usize) {
        if self.lanes[from].len() < n {
            panic!(
                "Tried to move {} from lane {} when it only has {} items",
                n,
                from,
                self.lanes[from].len()
            );
        }

        let mut items = Vec::with_capacity(n);

        for _ in 0..n {
            items.push(self.lanes[from].pop().unwrap());
        }

        for _ in 0..n {
            self.lanes[to].push(items.pop().unwrap());
        }
    }

    pub fn reverse_lanes(&mut self) {
        for lane in &mut self.lanes {
            lane.reverse();
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let (cargo, operations) = read_input(filename);

    println!("part 1: {}", part1(&cargo, &operations));
    println!("part 2: {}", part2(&cargo, &operations));
}

fn read_input(filename: &str) -> (Cargo, Vec<MoveOperation>) {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let num_lanes = contents.lines().next().unwrap().len() / 4;

    let mut cargo = Cargo::with_lanes(num_lanes);

    let mut lines = contents.lines();

    while let Some(line) = lines.next() {
        if line.starts_with(" 1 ") {
            break;
        }

        let mut chars = line.chars();
        for i in 0..num_lanes {
            chars.next(); // lanes have the format '[X] ' repeated. Skip the first '['
                          // Read the character
            if let Some(item) = chars.next() {
                if item != ' ' {
                    cargo.push_to_lane(i, item)
                }
            }

            // Skip the remaining '] '
            chars.next();
            chars.next();
        }
    }

    // We read the lanes in reverse so need to flip them
    cargo.reverse_lanes();

    let mut operations = Vec::new();
    // Now read the operations
    while let Some(line) = lines.next() {
        if line.len() > 0 {
            operations.push(MoveOperation::from_str(line))
        }
    }

    (cargo, operations)
}

fn part1(c: &Cargo, operations: &Vec<MoveOperation>) -> String {
    let mut cargo = c.clone();

    operations
        .iter()
        .for_each(|op| cargo.move_items(op.n, op.from - 1, op.to - 1));

    cargo
        .lanes
        .iter()
        .map(|lane| lane.last().unwrap_or(&' '))
        .collect()
}

fn part2(c: &Cargo, operations: &Vec<MoveOperation>) -> String {
    let mut cargo = c.clone();

    operations
        .iter()
        .for_each(|op| cargo.move_items_batch(op.n, op.from - 1, op.to - 1));

    cargo
        .lanes
        .iter()
        .map(|lane| lane.last().unwrap_or(&' '))
        .collect()
}
