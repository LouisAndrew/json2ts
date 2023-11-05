use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

pub fn get_buffer(path: &PathBuf) -> std::io::Result<BufReader<File>> {
    let file = File::open(path)?;
    Ok(BufReader::new(file))
}

pub fn get_matches(buffer: impl BufRead, pattern: &str) -> Vec<String> {
    let mut matches: Vec<String> = vec![];
    for line in buffer.lines() {
        match line {
            Ok(content) => {
                if content.contains(pattern) {
                    matches.push(content);
                }
            }
            Err(error) => {
                println!("Error: {}", error);
            }
        }
    }

    matches
}
