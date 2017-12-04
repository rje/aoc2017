use std::iter::FromIterator;
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let path = "data/real_data.txt";
    let lines = get_lines(&path);
    let result = count_valid_phrases(&lines);
    println!("Part 1: Found {} valid passwords", result);

    let result = count_valid_phrases_part_2(&lines);
    println!("Part 2: Found {} valid passwords", result);
}

fn get_lines(path: &str) -> Vec<String> {
    let file = BufReader::new(File::open(path).unwrap());
    return file.lines().map(|line| { line.unwrap() }).collect();
}

// part 1
fn count_valid_phrases(lines: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        let mut split_line : Vec<&str> = line.split_whitespace().collect();
        let pre_dedup_count = split_line.len();
        split_line.sort();
        split_line.dedup();
        if split_line.len() == pre_dedup_count {
            sum += 1;
        }
    }

    return sum;
}

// part 2
fn count_valid_phrases_part_2(lines: &Vec<String>) -> u32 {
    let mut sum = 0;

    for line in lines {
        let split_line : Vec<&str> = line.split_whitespace().collect();
        let mut sorted_line : Vec<String> = Vec::new();

        for i in 0 .. split_line.len() {
            let word = split_line[i];
            let mut chars : Vec<char> = word.chars().collect();
            chars.sort();
            sorted_line.push(String::from_iter(chars));
        }

        sorted_line.sort();
        let pre_dedup_count = sorted_line.len();
        sorted_line.dedup();
        if sorted_line.len() == pre_dedup_count {
            sum += 1;
        }
    }

    return sum;
}
