#[macro_use] extern crate lazy_static;
mod util;
mod dayn;
mod day1;
mod day2;
mod day3;
mod day4;

fn main(){

    let filename = "./inputs/day4.txt";
    day4::part1(filename);
    day4::part2(filename);
}