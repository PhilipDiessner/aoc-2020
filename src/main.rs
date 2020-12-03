#[macro_use] extern crate lazy_static;
mod util;
mod day1;
mod day2;
mod day3;
mod dayn;

fn main(){

    let filename = "./inputs/day3.txt";
    day3::part1(filename);
    day3::part2(filename);
}