use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let file_path = "input.txt";
    let lines = read_lines(file_path.to_string());
    let mut max_calories = 0;
    let mut max_index = 0;
    let mut elves_counter: i32 = 0;
    let mut calorie_counter = 0;
    for line in lines {
        let c = line.unwrap();
        if c.trim().is_empty() {
            if calorie_counter > max_calories {
                max_index = elves_counter;
                max_calories = calorie_counter;
            }
            calorie_counter = 0;
            elves_counter += 1;
        } else {
            calorie_counter += c.parse::<i32>().unwrap();
        }
    }
    println!("Elf {} had Max Calories {}", max_index, max_calories);
}
