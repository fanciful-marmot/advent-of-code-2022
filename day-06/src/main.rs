use std::collections::HashMap;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let sequence = read_input(filename);

    println!("part 1: {}", part1(&sequence));
    println!("part 2: {}", part2(&sequence));
}

fn read_input(filename: &str) -> String {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    String::from(contents.lines().next().unwrap())
}

fn part1(sequence: &str) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::new();

    const MARKER_SIZE: usize = 4;

    // Add first 3 characters to the set
    sequence.chars().take(MARKER_SIZE - 1).for_each(|c| {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    });

    for (i, c) in sequence.chars().enumerate().skip(MARKER_SIZE - 1) {
        // Add character to set
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);

        // Is everything unique?
        if counts.values().all(|&count| count <= 1) {
            return (i + 1) as u32;
        }

        let removal_char = &sequence.chars().skip(i - (MARKER_SIZE - 1)).next().unwrap();

        counts.insert(*removal_char, counts.get(removal_char).unwrap_or(&1) - 1);
    }

    0
}

fn part2(sequence: &str) -> u32 {
    let mut counts: HashMap<char, u32> = HashMap::new();

    const MARKER_SIZE: usize = 14;

    // Add first 3 characters to the set
    sequence.chars().take(MARKER_SIZE - 1).for_each(|c| {
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);
    });

    for (i, c) in sequence.chars().enumerate().skip(MARKER_SIZE - 1) {
        // Add character to set
        counts.insert(c, counts.get(&c).unwrap_or(&0) + 1);

        // Is everything unique?
        if counts.values().all(|&count| count <= 1) {
            return (i + 1) as u32;
        }

        let removal_char = &sequence.chars().skip(i - (MARKER_SIZE - 1)).next().unwrap();
        counts.insert(*removal_char, counts.get(removal_char).unwrap_or(&1) - 1);
    }

    0
}
