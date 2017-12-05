use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let path = "data/real_data.txt";
    let mut instr : Vec<i32> = load_data(path);
    let count = find_exit(&mut instr, &part_1_index_increment);
    println!("part 1 steps: {}", count);

    let mut instr : Vec<i32> = load_data(path);
    let count = find_exit(&mut instr, &part_2_index_increment);
    println!("part 2 steps: {}", count);
}

fn load_data(path: &str) -> Vec<i32> {
    let file = BufReader::new(File::open(path).unwrap());
    return file.lines().map(|line| { line.unwrap().parse::<i32>().unwrap() }).collect();
}

fn find_exit(instrs : &mut Vec<i32>, increment_func: &Fn(i32) -> i32) -> u32 {
    let mut count : u32 = 0;
    let mut index : i32 = 0;
    loop {
        if index < 0 || index >= instrs.len() as i32 {
            break;
        }
        let offset = instrs[index as usize];
        instrs[index as usize] += increment_func(offset);
        index += offset;
        count += 1;
    }

    return count;
}

fn part_1_index_increment(_current_offset: i32) -> i32 {
    return 1;
}

fn part_2_index_increment(current_offset: i32) -> i32 {
    if current_offset >= 3 { return -1; } else { return 1; }
}