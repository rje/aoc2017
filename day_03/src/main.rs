use std::collections::HashMap;

fn main() {
    let idx = 361527;
    let pos = find_manhattan_distance(idx);
    println!("Distance of {} is {}", idx, pos);

    let val = find_greater_than(idx);
    println!("First value greater than {} is {}", idx, val);
}

// part 1
fn find_manhattan_distance(idx : u32) -> i32 {
    let pos = calculate_coordinates(idx);
    return pos.0.abs() + pos.1.abs();
}

fn calculate_coordinates(idx : u32) -> (i32, i32) {
    if idx == 1 { return (0, 0); }
    let mut ring : u32 = 0;
    while idx > (2 * ring + 1).pow(2) {
        ring += 1;
    }
    let side_len = 2 * ring + 1;
    let mut rem = idx - (side_len - 2).pow(2) - 1;
    let mut pos : (i32, i32) = (ring as i32, -(ring as i32 - 1));

    pos = shift_coordinates(pos, &mut rem, side_len - 2, (0, 1));
    pos = shift_coordinates(pos, &mut rem, side_len - 1, (-1, 0));
    pos = shift_coordinates(pos, &mut rem, side_len - 1, (0, -1));
    pos = shift_coordinates(pos, &mut rem, side_len - 1, (1, 0));

    return pos;
}

fn shift_coordinates(pos: (i32, i32), rem : &mut u32, len : u32, dir : (i32, i32)) -> (i32, i32) {
    let amt : u32 = if *rem >= len {len} else {*rem};
    *rem -= amt;
    return (pos.0 + dir.0 * amt as i32, pos.1 + dir.1 * amt as i32);
}

// part 2
fn find_greater_than(value : u32) -> i32 {
    let mut cache : HashMap<(i32, i32), i32> = HashMap::new();
    cache.insert((0, 0), 1);

    let mut ring = 1;
    loop {
        let mut pos : (i32, i32) = (ring, -(ring - 1));
        let side_len = 2 * ring + 1;
        let mut results = walk_edge(&mut cache, &mut pos, side_len - 2, (0, 1));
        results.append(&mut walk_edge(&mut cache, &mut pos, side_len - 1, (-1, 0)));
        results.append(&mut walk_edge(&mut cache, &mut pos, side_len - 1, (0, -1)));
        results.append(&mut walk_edge(&mut cache, &mut pos, side_len, (1, 0)));
        results.sort();
        let found = results.into_iter().find(|&x| x > value as i32);
        if found.is_some() {
            return found.unwrap();
        }
        ring += 1;
    }
}

fn walk_edge(cache: &mut HashMap<(i32, i32), i32>, pos: &mut (i32, i32), len: i32, dir: (i32, i32)) -> Vec<i32> {
    let mut to_return : Vec<i32> = Vec::new();

    for i in 0 .. len {
        let sum = find_sum(cache, *pos);
        to_return.push(sum);
        cache.insert(*pos, sum);
        pos.0 = pos.0 + dir.0;
        pos.1 = pos.1 + dir.1;
    }

    return to_return;
}

fn find_sum(cache: &mut HashMap<(i32, i32), i32>, pos: (i32, i32)) -> i32 {
    let mut sum = 0;
    for x in (pos.0 - 1) .. (pos.0 + 2) {
        for y in (pos.1 - 1) .. (pos.1 + 2) {
            sum += cache.get(&(x, y)).unwrap_or(&0);
        }
    }
    return sum;
}