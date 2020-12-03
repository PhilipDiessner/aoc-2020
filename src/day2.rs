use crate::util;
use std::ops::Range;
use regex;

fn parse_line(inp: & str) -> regex::Captures{
    lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
    }
    if let Some(caps) = RE.captures_iter(inp).next() {
        caps
    } else { panic!() }
}

fn valid_pass(policy: (&str, &str, &str), pass: &str) -> bool{
    let c = pass.matches(policy.2).count() as u32;
    if let Ok(start) = policy.0.parse::<u32>() {
        if let Ok(end) = policy.1.parse::<u32>() {
            let range: Range<u32> = Range{ start, end: end+1};
    range.contains(&c)
        } else { panic!() }

    }else { panic!() }
}

fn valid_pass_two(policy: (&str, &str, &str), pass: &str) -> bool{
    let ch : Vec<char> = policy.2.chars().collect();
    let pass_vec: Vec<char> = pass.chars().collect();
    if let Ok(first) = policy.0.parse::<usize>(){
        if let Ok(second) = policy.1.parse::<usize>(){
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
                let caps= parse_line(inp.as_str());
                let (pol, pw) = ((&caps[1], &caps[2], &caps[3]), &caps[4]);
                res += valid_pass(pol, pw) as u32;

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
                let caps = parse_line(inp.as_str());
                let (pol, pw) = ((&caps[1], &caps[2], &caps[3]), &caps[4]);
                res += valid_pass_two(pol, pw) as u32;
            }
        }
        println!("{}",res)
    }
}