use std::collections::HashMap;
use crate::util;

pub fn part1(filename: &str) {
    let mut done: HashMap<u32, u32> = HashMap::new();
    let lines = util::read_lines(filename).unwrap();
        for line in lines {
            let ip = line.unwrap();
            let trim = ip.trim().parse::<u32>().unwrap();
            let rem = 2020 - trim;
            if !done.contains_key(&trim) {
                done.insert(rem, trim);
            } else {
                println!("first star: {}", trim * rem);
            }

        }
}

pub fn part2(filename: &str) {
    let mut single: HashMap<u32, u32> = HashMap::new();
    let mut double: HashMap<u32, (u32, u32)> = HashMap::new();
    let lines = util::read_lines(filename).unwrap();
    for line in lines {
        let ip = line.unwrap();
        let trim = ip.trim().parse::<u32>().unwrap();
        let rem = 2020 - trim;
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