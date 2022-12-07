use std::env;
use std::fs;

struct Range(u32, u32);

impl Range {
    pub fn from_str(s: &str) -> Self {
        let split: Vec<&str> = s.split('-').collect();

        Range(
            u32::from_str_radix(split[0], 10).unwrap(),
            u32::from_str_radix(split[1], 10).unwrap(),
        )
    }

    pub fn contains(&self, other: &Range) -> bool {
        self.0 <= other.0 && other.1 <= self.1
    }

    pub fn overlaps(&self, other: &Range) -> bool {
        self.contains(other)
            || (self.0 <= other.0 && self.1 >= other.0)
            || (self.0 <= other.1 && self.1 >= other.1)
            || other.contains(self)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let range_pairs = read_input(filename);

    println!("part 1: {}", part1(&range_pairs));
    println!("part 2: {}", part2(&range_pairs));
}

fn read_input(filename: &str) -> Vec<(Range, Range)> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents
        .lines()
        .map(|l| {
            let range_strs: Vec<&str> = l.split(',').collect();

            (
                Range::from_str(range_strs[0]),
                Range::from_str(range_strs[1]),
            )
        })
        .collect()
}

fn part1(range_pairs: &Vec<(Range, Range)>) -> u32 {
    range_pairs
        .iter()
        .map(|range_pair| {
            if range_pair.0.contains(&range_pair.1) || range_pair.1.contains(&range_pair.0) {
                1
            } else {
                0
            }
        })
        .sum()
}

fn part2(range_pairs: &Vec<(Range, Range)>) -> u32 {
    range_pairs
        .iter()
        .map(|range_pair| {
            if range_pair.0.overlaps(&range_pair.1) {
                1
            } else {
                0
            }
        })
        .sum()
}
