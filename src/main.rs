#[macro_use] extern crate lazy_static;
mod util;
mod dayn;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
//mod day7;
mod day8;

fn main(){

    let filename = "./inputs/day8.txt";
    day8::part1(filename);
    day8::part2(filename);
}