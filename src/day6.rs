use crate::util;
use std::collections::HashSet;
use reduce::Reduce;

pub fn part1(filename: &str) {
    let mut counts= 0;
    let str = util::filename_to_string(filename).unwrap();
    let lines = str.split("\n\n");
    for line in lines {
        let mut group = HashSet::new();
        for char in line.chars() {
                group.insert(char);
            }
        group.remove(&'\n');
        counts += group.len();
    }
    println!("{}", counts);
}

pub fn part2(filename: &str) {
    let mut counts= 0;
    let str = util::filename_to_string(filename).unwrap();
    let lines = str.split("\n\n");
    for line in lines {
        //let members :Vec<&str>=  line.split_terminator("\n").collect();
        let group = line.split_terminator("\n")
            .map(|member| member.chars().collect::<HashSet<char>>())
            .reduce(|a, b| a.intersection(&b).copied()
                .collect::<HashSet<char>>()).unwrap();
        counts += group.len();
    }
    println!("{}", counts);
}