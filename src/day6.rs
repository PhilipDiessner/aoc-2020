use crate::util;
use std::collections::HashSet;
use reduce::Reduce;

fn sub_part1(line:&str) -> usize {
    line.chars().filter(|&a| a !='\n'  ).collect::<HashSet<char>>().len()
}

fn sub_part2(line: &str) -> usize {
    line.split_terminator("\n")
            .map(|member| member.chars().collect::<HashSet<char>>())
            .reduce(|a, b| a.intersection(&b).copied()
                .collect::<HashSet<char>>()).unwrap().len()
}


pub fn part1(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let counts = str.split("\n\n").map(|line| sub_part1(line))
        .reduce(|a,b|a+b).unwrap();
    println!("{}", counts);
}

pub fn part2(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let counts = str.split("\n\n").map(|line| sub_part2(line))
        .reduce(|a,b|a+b).unwrap();
    println!("{}", counts);
}