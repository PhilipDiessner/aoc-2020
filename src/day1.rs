use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn day1() {
    let mut done: HashMap<u32, u32> = HashMap::new();
    let mut single: HashMap<u32, u32> = HashMap::new();
    let mut double: HashMap<u32, (u32, u32)> = HashMap::new();
    let filename = "./inputs/day1.txt";
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if let Ok(trim) = ip.trim().parse::<u32>() {
                    let rem = 2020 - trim;
                    if !done.contains_key(&trim) {
                        done.insert(rem, trim);
                    } else {
                        println!("first star: {}", trim * rem);
                    }
                    if !double.contains_key(&trim) {
                        for (&k, &v) in &single {
                            if trim < k {
                                double.insert(2020 - v - trim, (trim, v));
                            }
                        }
                        single.insert(rem, trim);
                    } else {
                        if let Some((a, b)) = double.get(&trim) {
                            println!("second star: {}", trim * a * b);
                        }
                    }
                }
            }
        }
    }
}