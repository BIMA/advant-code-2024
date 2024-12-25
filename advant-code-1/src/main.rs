use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut list1 = Vec::new();
    let mut list2 = Vec::new();

    let file_path = args[1].clone();

    let contents = fs::read_to_string(file_path)?;

    for line in contents.lines() {
        let mut line = line.split_whitespace();
        let first = line.next().unwrap();
        let second = line.next().unwrap();  
        list1.push(first.parse::<i32>().unwrap());
        list2.push(second.parse::<i32>().unwrap());
    }

    let mut list1_sorted = list1.clone();   
    list1_sorted.sort();

    let mut list2_sorted = list2.clone();
    list2_sorted.sort();

    let mut diff = 0;

    for i in 0..list1_sorted.len() {
        diff += (list1_sorted[i] - list2_sorted[i]).abs();
    }

    println!("{}", diff);

    Ok(()) // Return Ok(()) to indicate successful execution
}
