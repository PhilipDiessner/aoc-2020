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
mod day10;

fn main(){

    let filename = "./inputs/day10.txt";
    day10::part1(filename);
    day10::part2(filename);
}