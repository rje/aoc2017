use std::fs::File;
use std::io::prelude::*;
use std::io;

fn main() {
    let mut data = load_data("data/real_data.txt").unwrap();
    let results = count_steps_to_loop(&mut data);
    println!("Part 1 steps: {}", results);

    let mut data = load_data("data/real_data.txt").unwrap();
    let results = count_steps_to_repeat(&mut data);
    println!("Part 2 steps: {}", results);
}

fn load_data(path: &str) -> Result<Vec<i32>, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let results : Vec<i32> = contents.split_whitespace().map(|val| { val.parse::<i32>().unwrap() }).collect();
    return Ok(results);
}

fn advance_memory(banks: &mut Vec<i32>) {
    let mut idx :usize = 0;
    let mut max = -1;
    // wanted to use iter().enumerate() but can't because it works back to front!
    for i in 0 .. banks.len() {
        if banks[i] > max {
            max = banks[i];
            idx = i;
        }
    }
    let mut count = banks[idx];
    banks[idx] = 0;
    while count > 0 {
        idx = (idx + 1) % banks.len();
        banks[idx] += 1;
        count -= 1;
    }
}

// part 1
fn count_steps_to_loop(banks: &mut Vec<i32>) -> i32 {
    let mut count : i32 = 0;
    let mut prev : Vec<String> = Vec::new();

    loop {
        advance_memory(banks);
        let strings : Vec<_> = banks.iter().map(|&val| { let x = val.to_string(); return x; }).collect::<Vec<String>>();
        let hash_string = strings.concat();
        count += 1;
        if prev.contains(&hash_string) {
            break;
        }
        prev.push(hash_string);
    }
    return count;
}

// part 1
fn count_steps_to_repeat(banks: &mut Vec<i32>) -> i32 {
    let mut count : i32 = 0;
    let mut prev : Vec<String> = Vec::new();
    let repeat_to_find : String;

    loop {
        advance_memory(banks);
        let strings : Vec<_> = banks.iter().map(|&val| { let x = val.to_string(); return x; }).collect::<Vec<String>>();
        let hash_string = strings.concat();
        count += 1;
        if prev.contains(&hash_string) {
            repeat_to_find = hash_string;
            break;
        }
        prev.push(hash_string);
    }

    count = 0;
    loop {
        advance_memory(banks);
        let strings : Vec<_> = banks.iter().map(|&val| { let x = val.to_string(); return x; }).collect::<Vec<String>>();
        let hash_string = strings.concat();
        count += 1;
        if repeat_to_find == hash_string {
            break;
        }
    }
    return count;
}

