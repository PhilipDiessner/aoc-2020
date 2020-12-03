#[macro_use] extern crate lazy_static;
mod util;
mod day1;
mod day2;
mod dayn;

fn main(){

    let filename = "./inputs/day2.txt";
    day2::part1(filename);
    day2::part2(filename);
}