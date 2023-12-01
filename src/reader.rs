use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn read_input(file_name: &str) -> std::io::Result<Vec<String>> {
    let mut string_vec: Vec<String> = Vec::new();
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line?;
        string_vec.push(line);
    }
    Ok(string_vec)
}
