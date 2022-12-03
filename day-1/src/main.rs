use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let elves = read_input(filename);

    println!("part 1: {}", part1(&elves).unwrap());
    // println!("part 2: {}", part2(&numbers).unwrap());
}

fn read_input(filename: &str) -> Vec<Vec<u32>> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    let lines: Vec<Option<u32>> = contents
        .lines()
        .map(|l| match l.parse::<u32>() {
            Ok(n) => Some(n),
            _ => None,
        })
        .collect();

    let mut output = Vec::new();
    let mut current = Vec::new();

    for line in lines.iter() {
        match line {
            Some(n) => current.push(*n),
            None => {
                output.push(current);
                current = Vec::new();
            }
        }
    }

    output.push(current);

    return output;
}

// Finds the elf with the most calories, returns sum of calories
fn part1(values: &Vec<Vec<u32>>) -> Option<u32> {
    values.iter().map(|elf| elf.iter().sum()).max()
}
