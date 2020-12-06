#[macro_use] extern crate lazy_static;
mod util;
mod dayn;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

fn main(){

    let filename = "./inputs/day6.txt";
    day6::part1(filename);
    day6::part2(filename);
}