use std::fs::File;
use std::io::prelude::*;

// Takes a filename as input, returns a vector of tuples 
// Each tuple represents a set of edges in the text file
pub fn read_txt(filename: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(filename).expect("Could not open file");
    let reader = std::io::BufReader::new(file).lines();
    for line in reader {
        let msg = line.expect("Error reading");
        let v: Vec<&str> = msg.trim().split(' ').collect();
        let x: usize = v[0].parse::<usize>().unwrap();
        let y: usize = v[1].parse::<usize>().unwrap();
        result.push((x,y));
    }
    return result;
}

