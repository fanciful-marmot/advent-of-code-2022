use std::env;
use std::fs;

struct Sack {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Sack {
    pub fn from_str(line: &str) -> Self {
        let compartment_size = line.len() / 2;

        Sack {
            left: line
                .chars()
                .take(compartment_size)
                .map(|c| char_to_priority(c))
                .collect(),
            right: line
                .chars()
                .skip(compartment_size)
                .map(|c| char_to_priority(c))
                .collect(),
        }
    }

    pub fn contains(&self, item: u32) -> bool {
        self.left.contains(&item) || self.right.contains(&item)
    }

    pub fn find_duplicates(&self) -> Vec<u32> {
        let num_items = self.left.len();
        let mut sorted_left = self.left.clone();
        sorted_left.sort();
        let mut sorted_right = self.right.clone();
        sorted_right.sort();

        let mut output: Vec<u32> = Vec::new();

        let mut left_pointer: usize = 0;
        let mut right_pointer: usize = 0;
        let mut last_found_number: u32 = 0;
        while left_pointer < num_items && right_pointer < num_items {
            let left = sorted_left[left_pointer];
            let right = sorted_right[right_pointer];
            if left == right {
                // Avoid duplicates
                if left > last_found_number {
                    output.push(left);
                    last_found_number = left;
                }
                left_pointer += 1;
                right_pointer += 1;
            } else if left < right {
                left_pointer += 1;
            } else {
                right_pointer += 1;
            }
        }

        output
    }
}

fn char_to_priority(c: char) -> u32 {
    if c.is_uppercase() {
        ((c as u8) - 64u8) as u32 + 26
    } else {
        ((c as u8) - 96u8) as u32
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let sacks = read_input(filename);

    println!("part 1: {}", part1(&sacks));
    println!("part 2: {}", part2(&sacks));
}

fn read_input(filename: &str) -> Vec<Sack> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents.lines().map(|l| Sack::from_str(l)).collect()
}

fn part1(sacks: &Vec<Sack>) -> u32 {
    sacks
        .iter()
        .map(|round| *round.find_duplicates().first().unwrap())
        .sum()
}

fn part2(sacks: &Vec<Sack>) -> u32 {
    // Find the item shared by all elves
    (0..sacks.len() / 3)
        .into_iter()
        .map(|i| {
            // Return the badge for each group
            // Get the group
            let sack_group: Vec<&Sack> = sacks.iter().skip(i * 3).take(3).collect();

            for i in 1..53 {
                if sack_group[0].contains(i)
                    && sack_group[1].contains(i)
                    && sack_group[2].contains(i)
                {
                    return i;
                }
            }

            0
        })
        .sum()
}
