use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}

fn main() {
    let mut ret = read_lines("input.txt".to_string()).fold(vec![0], |mut v, e| {
        let e = e.unwrap();
        if !e.trim().is_empty() {
            let s = v.last_mut().unwrap();
            *s = *s + e.parse::<u32>().unwrap();
        } else {
            v.push(0);
        }
        return v;
    });
    ret.sort_unstable();
    println!("{}", ret.into_iter().rev().take(3).sum::<u32>());
}
