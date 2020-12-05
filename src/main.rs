#[macro_use] extern crate lazy_static;
mod util;
mod dayn;
mod day1;
mod day2;
mod day3;
mod day4;
mod day5;

fn main(){

    let filename = "./inputs/day5.txt";
    day5::part1(filename);
    day5::part2(filename);
}