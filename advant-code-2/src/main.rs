use std::env;
use std::fs;
use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();

    let contents = fs::read_to_string(file_path)?;

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        left_list.push(line.next().unwrap().parse::<i32>().unwrap());
        right_list.push(line.next().unwrap().parse::<i32>().unwrap());
    }

    let mut right_hmap = HashMap::new();

    for m in right_list.iter() {
        *right_hmap.entry(m).or_insert(0) += 1;
    }

    let mut similarity = 0;

    for m in left_list.iter() {
        if right_hmap.contains_key(m) {
            similarity += m * right_hmap[m];
        } else {
            similarity += 0;
        }
    }

    println!("{}", similarity);

    Ok(())
}
