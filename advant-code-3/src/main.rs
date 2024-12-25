use std::env;
use std::fs;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let file_path = args[1].clone();
    
    let contents = fs::read_to_string(file_path)?;

    let mut matrix = Vec::new();

    for line in contents.lines() {
        let row: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        matrix.push(row);
    }

    let mut safe_counter = 0;
    let mut counter = 0;
    let mut prev: i32 = 0;
    let mut is_decreasing = false;
    let mut is_increasing = false;

    for row in matrix.iter() {
        for (i, col) in row.iter().enumerate() {
            if i != 0 {
                let mut diff = prev - *col;

                if diff > 0 {
                    is_decreasing = true;
                } else if diff < 0 {
                    is_increasing = true;
                }

                diff = diff.abs();

                if is_decreasing && is_increasing {
                    safe_counter = 0;
                    break;
                }

                if diff > 3 || diff == 0{
                    safe_counter = 0;
                    break;
                } else {
                    safe_counter = 1;
                }
            }

            prev = col.clone();
        }
        if safe_counter == 1 {
            counter += 1;
        }

        is_decreasing = false;
        is_increasing = false;
    }

    println!("{}", counter);

    Ok(())
}
