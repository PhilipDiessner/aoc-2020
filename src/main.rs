#[macro_use] extern crate lazy_static;
mod util;
mod dayn;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

fn main(){

    let filename = "./inputs/day7.txt";
    day7::part1(filename);
    day7::part2(filename);
}