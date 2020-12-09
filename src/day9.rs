use crate::util;
use std::env::temp_dir;
use reduce::Reduce;

struct XMAS<'a> {
    text: &'a Vec<u64>,
    preamble: usize,
}

impl XMAS<'_> {
    fn check_pos(&self, pos: usize) -> (bool, u64) {
        let val = self.text[pos];
        let mut res = false;
        for (i, first) in (pos - self.preamble..pos).enumerate() {
            for second in (pos - self.preamble + i..pos) {
                if self.text[first] + self.text[second] == val {
                    res = true;
                    break;
                }
            }
            if res { break; }
        }
        (res, val)
    }
}

pub fn part1(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let mut numbers: Vec<u64> = str.lines().map(|line| line.parse::<u64>().unwrap()).collect();
    let size = numbers.len();
    let xmas = XMAS { text: &numbers, preamble: 25 };
    let mut bad = 0;
    for i in xmas.preamble..size {
        let (res, val) = xmas.check_pos(i);
        if !res {
            bad = i;
            break;
        }
    }
    println!("{}, {}", numbers[bad], bad);
    let mut summed: Vec<u64> = vec![];
    let mut res = 0;
    for i in 0..bad {
        summed.push(numbers[..i].iter().sum())
    }
    for (i, &a) in summed[..].iter().enumerate() {
        for (j, &b) in summed[(i + 1)..].iter().enumerate() {
                //println!("{},{},{},{}", a,b, i,j);
            if b-a == numbers[bad] {
                println!("{},{},{}", numbers[i..(i+j+1)].iter().max().unwrap() + numbers[i..(i+j+1)].iter().min().unwrap() ,i,j);
                break;
            }
        }
    }
}

pub fn part2(filename: &str) {}