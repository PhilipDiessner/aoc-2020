use crate::util;
use reduce::Reduce;

fn run_part1(input: &String, left: usize, down: usize)->u32{
    let map = util::read_map(input.as_str());
    let mut pos = 0;
    let mut res :u32 = 0;
    println!("{},{}",map.len(), map[0].len());
    let to_iter = match down {
        1 =>  map,
        _ => map.into_iter().enumerate().filter(|&(i, _ )| i % down == 0 ).map(|(_, v)| v).collect()
    };
    for line in to_iter {
        if line[pos] == '#'{ res+=1; }
        pos = (pos+left) % line.len();
    }
    res
}

pub fn part1(filename: &str) {
    let input = util::filename_to_string(filename).unwrap();
    println!("{}", run_part1(&input, 3, 1));
}

pub fn part2(filename: &str) {
    let input = util::filename_to_string(filename).unwrap();
    let res =vec![run_part1(&input, 1, 1),
        run_part1(&input, 3, 1),
        run_part1(&input, 5, 1),
        run_part1(&input, 7, 1),
        run_part1(&input, 1, 2),];
    let sum = res.into_iter().reduce(|a, b| a*b);
    println!("{}", sum.unwrap());

}