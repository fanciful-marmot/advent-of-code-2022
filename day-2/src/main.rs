use std::env;
use std::fs;

struct Round(char, char);

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    println!("Reading input file {}", filename);
    let rounds = read_input(filename);

    println!("part 1: {}", part1(&rounds));
    println!("part 2: {}", part2(&rounds));
}

fn rps_score_part1(round: &Round) -> u32 {
    let score = match round {
        Round(c, 'X') => {
            1 + match c {
                'B' => 0,
                'C' => 6,
                _ => 3,
            }
        }
        Round(c, 'Y') => {
            2 + match c {
                'A' => 6,
                'C' => 0,
                _ => 3,
            }
        }
        Round(c, 'Z') => {
            3 + match c {
                'A' => 0,
                'B' => 6,
                _ => 3,
            }
        }
        _ => 0,
    };

    score
}

fn rps_score_part2(round: &Round) -> u32 {
    let score = match round {
        Round(c, 'X') => {
            0 + match c {
                'A' => 3,
                'B' => 1,
                _ => 2,
            }
        }
        Round(c, 'Y') => {
            3 + match c {
                'A' => 1,
                'B' => 2,
                _ => 3,
            }
        }
        Round(c, 'Z') => {
            6 + match c {
                'A' => 2,
                'B' => 3,
                _ => 1,
            }
        }
        _ => 0,
    };

    score
}

fn read_input(filename: &str) -> Vec<Round> {
    let contents = fs::read_to_string(filename).expect("Failed to read file");

    contents
        .lines()
        .map(|l| Round(l.chars().next().unwrap(), l.chars().skip(2).next().unwrap()))
        .collect()
}

fn part1(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(|round| rps_score_part1(round)).sum()
}

fn part2(rounds: &Vec<Round>) -> u32 {
    rounds.iter().map(|round| rps_score_part2(round)).sum()
}
