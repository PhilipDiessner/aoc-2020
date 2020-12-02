use crate::util;
use std::ops::Range;

fn split_to_vec<'a>(inp: &'a str, delim: & str) -> Vec<&'a str> {
    inp.split(delim).collect::<Vec<_>>()
}

fn valid_pass(policy: &str, pass: &str) -> bool{
    let a= split_to_vec(policy, " ");
    let c = pass.matches(a[1]).count() as u32;
    let d = split_to_vec(a[0], "-");
    if let Ok(start) = d[0].parse::<u32>() {
        if let Ok(end) = d[1].parse::<u32>() {
            let range: Range<u32> = Range{ start, end: end+1};
    range.contains(&c)
        } else { panic!() }

    }else { panic!() }
}

fn valid_pass_two(policy: &str, pass: &str) -> bool{
    let a= split_to_vec(policy, " ");
    let ch : Vec<char> = a[1].chars().collect();
    let d = split_to_vec(a[0], "-");
    let pass_vec: Vec<char> = pass.chars().collect();
    if let Ok(first) = d[0].parse::<usize>(){
        if let Ok(second) = d[1].parse::<usize>(){
                (pass_vec[first - 1] == ch[0] || pass_vec[second - 1] == ch[0]) &&
                    (pass_vec[first - 1] != pass_vec[second - 1])
        } else { panic!() }
    } else { panic!() }
}

pub fn part1(filename: &str) {
    if let Ok(lines) = util::read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut res = 0;
        for line in lines {
            if let Ok(inp) = line{
                let split = split_to_vec(inp.as_str(), ": ");
                res += valid_pass(split[0], split[1]) as u32;

            }
        }
        println!("{}",res)
    }
}

pub fn part2(filename: &str) {
    if let Ok(lines) = util::read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        let mut res = 0;
        for line in lines {
            if let Ok(inp) = line{
                let split = split_to_vec(inp.as_str(), ": ");
                res += valid_pass_two(split[0], split[1]) as u32;
            }
        }
        println!("{}",res)
    }
}