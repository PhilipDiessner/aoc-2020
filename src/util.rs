use std::fs;
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
    where P: AsRef<Path>, {
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn filename_to_string<P>(filename: P) -> io::Result<String>
    where P: AsRef<Path>, {
    fs::read_to_string(filename)
}


pub fn read_map(s: &str) -> Vec<Vec<char>> {
    s.lines().map(|line| {
        //line.split("").filter(|s| !s.is_empty()).collect()
        line.chars().collect()
    }).collect()
}
