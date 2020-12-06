use crate::util;
use std::collections;

pub fn part1(filename: &str) {
    let mut counts= 0;
    let str = util::filename_to_string(filename).unwrap();
    let lines = str.split("\n\n");
    for line in lines {
        let mut group = collections::HashSet::new();
        for char in line.chars() {
            if char != '\n' {
                group.insert(char);
            }
        }
        counts += group.len();
    }
    println!("{}", counts);
}

pub fn part2(filename: &str) {
    let mut counts= 0;
    let str = util::filename_to_string(filename).unwrap();
    let lines = str.split("\n\n");
    for line in lines {
        let mut group:collections::HashMap<char, u32> = collections::HashMap::new();
        let members :Vec<&str>=  line.split_terminator("\n").collect();
        let n_members = members.len() as u32;
        for member in members{
            for char in member.chars() {
                group.insert(char, if group.contains_key(&char)
                {group[&char]+1 } else { 1 });
            }
        }
        for &value in group.values(){
            if value== n_members {counts +=1}
        }
    }
    println!("{}", counts);
}