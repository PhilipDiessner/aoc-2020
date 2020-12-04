use crate::util;
use regex;
use std::collections::HashMap;

fn split_passports(passports: &str) -> Vec<&str> {
    passports.split("\n\n").collect()
}


fn parse_passport(inp: &str) -> HashMap<String, String> {
    lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?P<key>\w+):(?P<value>#?\w+) ?\n?").unwrap();
    }
    let mut pass : HashMap<String, String> = HashMap::new();
    for capture in RE.captures_iter(inp) {
        pass.insert(capture["key"].to_string(), capture["value"].to_string());
    }
    pass
}

fn valid_part1(passport: &HashMap<String, String>) -> bool {
        passport.contains_key("byr") &&
            passport.contains_key("iyr") &&
            passport.contains_key("eyr") &&
            passport.contains_key("hgt") &&
            passport.contains_key("hcl") &&
            passport.contains_key("ecl") &&
            passport.contains_key("pid")
}

fn valid_yr(yr: &String, start:u32, end: u32) -> bool {
    let number = yr.parse::<u32>();
    match number {
        Err(_) => false,
        Ok(value) => (start <= value && value <= end)
    }
}

fn valid_hgt(hgt: &String) -> bool {
    lazy_static! {
    static ref hgtre: regex::Regex = regex::Regex::new(r"(\d{2,3})(in|cm)").unwrap();
    }
    let caps = hgtre.captures(&hgt);
    match caps {
        None =>false,
        Some(res) => {
            let h = res.get(1).unwrap().as_str().parse::<u32>().unwrap();
            let unit = res.get(2).unwrap().as_str();
            println!("{}",hgt);
            (unit == "cm" && 150 <= h && h <= 193) || (unit == "in" && 59 <= h && h <= 76)
        }
    }
}

fn valid_hcl(pid: &String) -> bool {
    lazy_static! {
    static ref hclre: regex::Regex = regex::Regex::new(r"#[0-9a-f]{6}").unwrap();
    }
    let caps = hclre.captures(&pid);
    match caps {
        None =>false,
        Some(_) => true
    }
}

fn valid_ecl(pid: &String) -> bool {
    lazy_static! {
    static ref eclre: regex::Regex = regex::Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
    }
    let caps = eclre.captures(&pid);
    match caps {
        None =>false,
        Some(_) => true
    }
}

fn valid_pid(pid: &String) -> bool {
    lazy_static! {
    static ref pidre: regex::Regex = regex::Regex::new(r"^\d{9}$").unwrap();
    }
    let caps = pidre.captures(&pid);
    match caps {
        None =>false,
        Some(_) => true
    }
}


fn valid_part2(passport: &HashMap<String, String>) -> bool {
    valid_yr(passport.get("byr").unwrap(), 1920, 2002) &&
        valid_yr(passport.get("iyr").unwrap(), 2010, 2020) &&
        valid_yr(passport.get("eyr").unwrap(), 2020, 2030) &&
        valid_hgt(passport.get("hgt").unwrap()) &&
        valid_hcl(passport.get("hcl").unwrap()) &&
        valid_ecl(passport.get("ecl").unwrap()) &&
        valid_pid(passport.get("pid").unwrap())
}

pub fn part1(filename: &str) {
    let mut res1 = 0;
    let mut res2 = 0;
    let inp = util::filename_to_string(filename).unwrap();
    let passports = split_passports(inp.as_str());
    let relevant = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]; //, "cid"
    println!("{}", passports.len());
    for passport in passports {
        let parsed = parse_passport(passport);
        if valid_part1(&parsed){
            res1 += 1;
            if valid_part2(&parsed){ res2 += 1 }
        }
    }
    println!("{}, {}", res1, res2);
}

pub fn part2(filename: &str) {

}