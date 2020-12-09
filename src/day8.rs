use crate::util;
use regex::Regex;
use std::collections::HashSet;
use std::ops::Range;

lazy_static! {
    static ref RE: regex::Regex = regex::Regex::new(r"(?P<ins>\w+) (?P<arg>.\d+)\n").unwrap();
    }


#[derive(Debug, Clone)]
struct Instruct {
    code: String,
    arg: i32
}
#[derive(Debug)]
struct State<'a> {
    instr: &'a Vec<Instruct>,
    acc: i32,
    step: usize,
    history: HashSet<usize>
}

impl State<'_> {
    fn do_step (&mut self) {
        let ins = &(self.instr)[self.step];
        self.history.insert(self.step);
        match ins.code.as_str() {
            "acc" => {
                        self.acc+=ins.arg;
                        self.step+=1;
                    }
            "jmp" => self.step=(self.step as i32 + ins.arg) as usize,
            "nop" => self.step+=1,
            _ =>panic!()
        }
    }

    fn do_steps (&mut self) -> bool {
        let mut result = false;
        loop {
            self.do_step();
            if self.history.contains(&self.step){
                //println!("looping {}, {}",self.step, self.acc);
                break
            } else if self.step >= self.instr.len() {
                println!("exited {}, {}",self.step, self.acc);
                result = true;
                break
            }
        }
        result
    }
}

fn get_instr(inp: &String) -> Vec<Instruct> {
    let mut instr = vec![];
    for capture in RE.captures_iter(inp.as_str()) {
        let ins = Instruct {
            code: capture["ins"].parse().unwrap(),
            arg: capture["arg"].parse::<i32>().unwrap()
        };
        instr.push(ins);
    }
    instr
}

pub fn part1(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let mut boot = State{instr: &get_instr(&str), acc: 0, step: 0, history: HashSet::new()};
    boot.do_steps();
}

pub fn part2(filename: &str) {
    let str = util::filename_to_string(filename).unwrap();
    let mut instr = get_instr(&str);
    for i in 0..instr.len() {
        let new = get_instr(&str);
        let old = instr[i].clone();

        if old.code == "jmp" {
            instr[i].code = String::from("nop")
        } else if old.code == "nop"{
            instr[i].code = String::from("jmp")
        }
        if old.code == "jmp" || old.code == "nop" {
            let mut boot = State { instr: &instr, acc: 0, step: 0, history: HashSet::new() };
            if boot.do_steps() { break }
        }
        instr[i] = old;
    }

}